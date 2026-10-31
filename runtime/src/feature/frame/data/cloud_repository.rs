use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::feature::frame::data::channels::FrameChannels;
use crate::feature::frame::data::db::db_source;
use crate::feature::frame::data::key::{
    GATEWAY_FRAME, GATEWAY_FRAME_PRODUCTION, GATEWAY_FRAME_PRODUCTION_CHECKSUM, error_code,
};
use crate::feature::frame::data::logging;
use crate::feature::frame::data::memory::memory_source::MemoryFrameSource;
use crate::feature::frame::data::network::{binary_source, graphql_source};
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::feature::frame::domain::repository::{FrameRepository, FrameResult};
use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::{BinaryError, GatewayType, HttpClient, with_headers};
use crate::library::result::NBResult;
use crate::plugin::config::{self, ResolvedGatewayModel};
use crate::plugin::logger::NativeLoggerProvider;

pub(crate) enum SyncOutcome {
    Updated(NativeFrameModel),
    Unchanged,
}

pub(crate) struct CloudFrameRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    memory: MemoryFrameSource,
    channels: FrameChannels,
}

impl CloudFrameRepository {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
        config_client: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            cache,
            config_client,
            logger,
            memory: MemoryFrameSource::new(),
            channels: FrameChannels::new(),
        };
    }

    async fn from_cache(&self, route: &str) -> Option<Arc<NativeFrameModel>> {
        if let Some(frame) = self.memory.get_frame(route) {
            return Some(frame);
        }
        let frame = db_source::get_frame(
            self.cache.as_ref(),
            route,
            self.environment.development_mode(),
        )
        .await
        .ok()?;
        let frame = Arc::new(frame);
        self.memory.save_frame(route, frame.clone());
        return Some(frame);
    }

    async fn fetch(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<SyncOutcome> {
        let result = self.download_frame(route, parameters).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }
        return result;
    }

    fn store(&self, route: &str, frame: NativeFrameModel) -> Arc<NativeFrameModel> {
        let frame = Arc::new(frame);
        self.memory.save_frame(route, frame.clone());
        return frame;
    }

    async fn download_frame(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<SyncOutcome> {
        if self.environment.development_mode() {
            let gateway = self.config_client.gateway(GATEWAY_FRAME).await?;
            let frame = self
                .development_frame(&gateway, route, parameters)
                .await
                .map_err(|error| error.or_code(error_code::FRAME_DEV_SYNC))?;
            db_source::save_frame(self.cache.as_ref(), route, &frame, false).await?;
            return Ok(SyncOutcome::Updated(frame));
        }

        let gateway = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION).await?;
        let cached_checksum = db_source::cached_checksum(self.cache.as_ref(), route).await?;
        return match GatewayType::of(&gateway.gateway.gateway_type) {
            GatewayType::Binary => {
                self.binary_production_frame(&gateway, route, parameters, cached_checksum)
                    .await
            }
            GatewayType::GraphQl | GatewayType::Rest => {
                self.graphql_production_frame(&gateway, route, parameters, cached_checksum)
                    .await
            }
        };
    }

    async fn development_frame(
        &self,
        gateway: &ResolvedGatewayModel,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        return match GatewayType::of(&gateway.gateway.gateway_type) {
            GatewayType::Binary => {
                self.binary_development_frame(gateway, route, parameters)
                    .await
            }
            GatewayType::GraphQl | GatewayType::Rest => {
                self.graphql_development_frame(gateway, route, parameters)
                    .await
            }
        };
    }

    async fn binary_development_frame(
        &self,
        gateway: &ResolvedGatewayModel,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let result = binary_source::frame(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.gateway.value,
            route,
        )
        .await;
        return match result {
            Ok(frame) => Ok(frame),
            Err(error) => {
                self.fallback(route, error)?;
                self.graphql_development_frame(gateway, route, parameters)
                    .await
            }
        };
    }

    async fn graphql_development_frame(
        &self,
        gateway: &ResolvedGatewayModel,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        return graphql_source::frame(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.endpoint,
            route,
            parameters,
        )
        .await;
    }

    async fn binary_production_frame(
        &self,
        gateway: &ResolvedGatewayModel,
        route: &str,
        parameters: &HashMap<String, String>,
        cached_checksum: Option<String>,
    ) -> NBResult<SyncOutcome> {
        let result = binary_source::frame_production(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.gateway.value,
            route,
            parameters,
            cached_checksum.as_deref(),
        )
        .await;
        return match result {
            Ok(Some(frame)) => {
                db_source::save_frame(self.cache.as_ref(), route, &frame, true).await?;
                Ok(SyncOutcome::Updated(frame))
            }
            Ok(None) => Ok(SyncOutcome::Unchanged),
            Err(error) => {
                self.fallback(route, error)
                    .map_err(|error| error.or_code(error_code::FRAME_PRODUCTION_SYNC))?;
                self.graphql_production_frame(gateway, route, parameters, cached_checksum)
                    .await
            }
        };
    }

    async fn graphql_production_frame(
        &self,
        gateway: &ResolvedGatewayModel,
        route: &str,
        parameters: &HashMap<String, String>,
        cached_checksum: Option<String>,
    ) -> NBResult<SyncOutcome> {
        if let Some(cached) = cached_checksum {
            let checksum_gateway = self
                .config_client
                .gateway(GATEWAY_FRAME_PRODUCTION_CHECKSUM)
                .await?;
            let remote = graphql_source::frame_production_checksum(
                self.http.as_ref(),
                self.headers(&checksum_gateway),
                &checksum_gateway.endpoint,
                route,
                parameters,
            )
            .await
            .map_err(|error| error.or_code(error_code::FRAME_CHECKSUM))?;
            if remote == cached {
                return Ok(SyncOutcome::Unchanged);
            }
        }
        let frame = graphql_source::frame_production(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.endpoint,
            route,
            parameters,
        )
        .await
        .map_err(|error| error.or_code(error_code::FRAME_PRODUCTION_SYNC))?;
        db_source::save_frame(self.cache.as_ref(), route, &frame, true).await?;
        return Ok(SyncOutcome::Updated(frame));
    }

    fn fallback(&self, route: &str, error: BinaryError) -> NBResult<()> {
        let error = match error {
            BinaryError::Failed(error) => return Err(error),
            BinaryError::Fallback(error) => error,
        };
        logging::log_binary_fallback(&self.logger, &self.sdk_config, route, &error);
        return Ok(());
    }

    fn headers(&self, gateway: &ResolvedGatewayModel) -> HashMap<String, String> {
        return with_headers(&self.environment, &self.sdk_config, &gateway.install_id);
    }
}

#[async_trait::async_trait]
impl FrameRepository for CloudFrameRepository {
    async fn load(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()> {
        if let Some(frame) = self.from_cache(route).await {
            self.channels.publish(route, Ok(frame));
            let _ = self.sync(route, parameters).await;
            return Ok(());
        }
        match self.fetch(route, parameters).await {
            Ok(SyncOutcome::Updated(frame)) => {
                let frame = self.store(route, frame);
                self.channels.publish(route, Ok(frame));
                Ok(())
            }
            Ok(SyncOutcome::Unchanged) => {
                let error = db_source::not_cached();
                self.channels.publish(route, Err(error.clone()));
                Err(error)
            }
            Err(error) => {
                self.channels.publish(route, Err(error.clone()));
                Err(error)
            }
        }
    }

    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()> {
        match self.fetch(route, parameters).await? {
            SyncOutcome::Updated(frame) => {
                let frame = self.store(route, frame);
                if self.environment.development_mode() {
                    self.channels.publish(route, Ok(frame));
                }
            }
            SyncOutcome::Unchanged => {}
        }
        return Ok(());
    }

    fn subscribe(&self, route: &str) -> watch::Receiver<FrameResult> {
        return self
            .channels
            .subscribe(route, || match self.memory.get_frame(route) {
                Some(frame) => Ok(frame),
                None => Err(db_source::not_cached()),
            });
    }

    async fn clear(&self, route: &str) -> NBResult<()> {
        self.memory.clear(route);
        return db_source::clear(self.cache.as_ref(), route).await;
    }

    async fn clear_all(&self) -> NBResult<()> {
        self.memory.clear_all();
        return db_source::clear_all(self.cache.as_ref()).await;
    }
}

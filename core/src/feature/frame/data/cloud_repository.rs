use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::feature::frame::data::channels::FrameChannels;
use crate::feature::frame::data::db::db_source;
use crate::feature::frame::data::network::cloud_source::{self, SyncOutcome};
use crate::feature::frame::data::key::{
    GATEWAY_FRAME, GATEWAY_FRAME_PRODUCTION, GATEWAY_FRAME_PRODUCTION_CHECKSUM,
};
use crate::feature::frame::data::logging;
use crate::feature::frame::data::memory::memory_source::MemoryFrameSource;
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::feature::frame::domain::repository::{FrameRepository, FrameResult};
use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;
use crate::plugin::config;
use crate::plugin::logger::NativeLoggerProvider;

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

    fn from_cache(&self, route: &str) -> Option<Arc<NativeFrameModel>> {
        if let Some(frame) = self.memory.get_frame(route) {
            return Some(frame);
        }
        let frame = db_source::get_frame(
            self.cache.as_ref(),
            route,
            self.environment.development_mode(),
        )
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
        let frame = self.config_client.gateway(GATEWAY_FRAME).await?;
        let production = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION).await?;
        let checksum = self
            .config_client
            .gateway(GATEWAY_FRAME_PRODUCTION_CHECKSUM)
            .await?;
        return cloud_source::sync_cloud(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            self.cache.as_ref(),
            frame.gateway,
            production.gateway,
            checksum.gateway,
            &frame.endpoint,
            &frame.install_id,
            route,
            parameters,
        )
        .await;
    }
}

#[async_trait::async_trait]
impl FrameRepository for CloudFrameRepository {
    async fn load(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()> {
        if let Some(frame) = self.from_cache(route) {
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

    async fn clear_all(&self, routes: &[String]) -> NBResult<()> {
        self.memory.clear_all(routes);
        return db_source::clear_all(self.cache.as_ref(), routes).await;
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::frame::data::channels::FrameChannels;
use crate::frame::data::cloud_source;
use crate::frame::data::db_source;
use crate::frame::data::key::{GATEWAY_FRAME, GATEWAY_FRAME_PRODUCTION, GATEWAY_FRAME_PRODUCTION_CHECKSUM};
use crate::frame::data::logging;
use crate::frame::data::memory_source::MemoryFrameSource;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::domain::repository::{FrameRepository, FrameUpdate};

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
        let frame = db_source::get_frame(self.cache.as_ref(), route, self.environment.development_mode()).ok()?;
        let frame = Arc::new(frame);
        self.memory.save_frame(route, frame.clone());
        return Some(frame);
    }

    async fn from_network(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> FrameUpdate {
        let result = self.download_frame(route, parameters).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }
        let frame = Arc::new(result?);
        self.memory.save_frame(route, frame.clone());
        return Ok(frame);
    }

    async fn download_frame(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let frame = self.config_client.gateway(GATEWAY_FRAME).await?;
        let production = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION).await?;
        let checksum = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION_CHECKSUM).await?;
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
        let result = self.from_network(route, parameters).await;
        self.channels.publish(route, result.clone());
        return result.map(|_| ());
    }

    async fn sync(&self, route: &str, parameters: &HashMap<String, String>) -> NBResult<()> {
        let frame = self.from_network(route, parameters).await?;
        if self.environment.development_mode() {
            self.channels.publish(route, Ok(frame));
        }
        return Ok(());
    }

    fn subscribe(&self, route: &str) -> watch::Receiver<FrameUpdate> {
        return self.channels.subscribe(route, || match self.memory.get_frame(route) {
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

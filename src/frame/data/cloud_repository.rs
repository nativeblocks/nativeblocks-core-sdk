use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::frame::data::key::{
    GATEWAY_FRAME, GATEWAY_FRAME_PRODUCTION, GATEWAY_FRAME_PRODUCTION_CHECKSUM,
};
use crate::frame::data::logging;
use crate::frame::data::source;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::domain::repository::FrameRepository;

/// Cloud-backed [`FrameRepository`]. Resolves gateways through the config client
/// and reads the dev-vs-production decision from the environment internally — so
/// the dev/prod axis lives here, never leaking up to the use cases.
pub(crate) struct CloudFrameRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
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
        };
    }

    async fn fetch(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        // The cloud pipeline needs three gateways resolved from the project config;
        // they share the same endpoint and install id.
        let frame = self.config_client.gateway(GATEWAY_FRAME).await?;
        let production = self.config_client.gateway(GATEWAY_FRAME_PRODUCTION).await?;
        let checksum = self
            .config_client
            .gateway(GATEWAY_FRAME_PRODUCTION_CHECKSUM)
            .await?;
        return source::sync_cloud(
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
    async fn sync(
        &self,
        route: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let result = self.fetch(route, parameters).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }
        return result;
    }

    fn get(&self, route: &str) -> NBResult<NativeFrameModel> {
        return source::get_frame(self.cache.as_ref(), route, self.environment.development_mode());
    }

    fn clear(&self, route: &str) -> NBResult<()> {
        return source::clear(self.cache.as_ref(), route);
    }

    fn clear_all(&self, routes: &[String]) -> NBResult<()> {
        return source::clear_all(self.cache.as_ref(), routes);
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::{ErrorModel, NBResult};
use crate::frame::data::key::error_code;
use crate::frame::data::logging;
use crate::frame::data::source;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::domain::repository::FrameRepository;

pub(crate) struct CommunityFrameRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

impl CommunityFrameRepository {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            cache,
            logger,
        };
    }

    async fn fetch(&self, route: &str) -> NBResult<NativeFrameModel> {
        let endpoint = self
            .environment
            .community_frame_endpoint(route)
            .ok_or_else(|| {
                ErrorModel::network(format!("No community frame endpoint configured for '{route}'"))
                    .with_code(error_code::FRAME_COMMUNITY_SYNC)
            })?;
        return source::sync_community(self.http.as_ref(), self.cache.as_ref(), &endpoint, route)
            .await;
    }
}

#[async_trait::async_trait]
impl FrameRepository for CommunityFrameRepository {
    async fn sync(
        &self,
        route: &str,
        _parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let result = self.fetch(route).await;
        match &result {
            Ok(_) => logging::log_sync_success(&self.logger, &self.sdk_config, route),
            Err(error) => logging::log_sync_failure(&self.logger, &self.sdk_config, route, error),
        }
        return result;
    }

    fn get(&self, route: &str) -> NBResult<NativeFrameModel> {
        return source::get_frame(self.cache.as_ref(), route, false);
    }

    fn clear(&self, route: &str) -> NBResult<()> {
        return source::clear(self.cache.as_ref(), route);
    }

    fn clear_all(&self, routes: &[String]) -> NBResult<()> {
        return source::clear_all(self.cache.as_ref(), routes);
    }
}

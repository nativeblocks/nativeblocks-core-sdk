use std::collections::HashMap;
use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::di;
use crate::frame::presenter::logging::FrameLogger;
use crate::frame::presenter::state_manager::FrameStateManager;

#[derive(uniffi::Object)]
pub struct FrameClient {
    container: Arc<di::Container>,
    services: Arc<di::Services>,
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NBError> {
        let container = di::get_or_create(&environment, &config)?;
        let services = container.services(http, cache);
        return Ok(Arc::new(Self { container, services }));
    }

    pub fn state_manager(&self) -> Arc<FrameStateManager> {
        let logger = FrameLogger::new(
            self.container.logger(),
            self.container.sdk_config().clone(),
        );
        return FrameStateManager::new(
            self.services.frame_repository(),
            self.container.global_parameters(),
            logger,
        );
    }

    pub async fn sync_frame(&self, route: String, parameters: HashMap<String, String>) -> Result<(), NBError> {
        return self
            .services
            .frame_repository()
            .sync(&route, &parameters)
            .await
            .map_err(NBError::from);
    }

    pub async fn clear(&self, route: String) -> Result<(), NBError> {
        return self
            .services
            .frame_repository()
            .clear(&route)
            .await
            .map_err(NBError::from);
    }

    pub async fn clear_all(&self, routes: Vec<String>) -> Result<(), NBError> {
        return self
            .services
            .frame_repository()
            .clear_all(&routes)
            .await
            .map_err(NBError::from);
    }
}

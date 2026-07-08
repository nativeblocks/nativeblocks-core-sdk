use std::collections::HashMap;
use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::frame::di::container::Container;
use crate::frame::presenter::state_manager::FrameStateManager;

#[derive(uniffi::Object)]
pub struct FrameClient {
    container: Arc<Container>,
    environment: NativeblocksEnvironment,
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
        environment.validate()?;
        let container = Arc::new(Container::new(
            environment.clone(),
            config.clone(),
            http,
            cache,
        )?);
        return Ok(Arc::new(Self {
            container,
            environment,
        }));
    }

    pub fn state_manager(&self) -> Arc<FrameStateManager> {
        return FrameStateManager::new(
            self.container.repository_arc(),
            self.environment.instance_name().to_string(),
        );
    }

    pub async fn sync_frame(&self, route: String, parameters: HashMap<String, String>) -> Result<(), NBError> {
        return self
            .container
            .repository()
            .sync(&route, &parameters)
            .await
            .map_err(NBError::from);
    }

    pub async fn clear(&self, route: String) -> Result<(), NBError> {
        return self
            .container
            .repository()
            .clear(&route)
            .await
            .map_err(NBError::from);
    }

    pub async fn clear_all(&self, routes: Vec<String>) -> Result<(), NBError> {
        return self
            .container
            .repository()
            .clear_all(&routes)
            .await
            .map_err(NBError::from);
    }
}

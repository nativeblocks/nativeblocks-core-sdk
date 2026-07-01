use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::localization::di::container::Container;
use crate::localization::domain::model::NativeLocalizationModel;
use crate::localization::presenter::state_manager::LocalizationStateManager;

#[derive(uniffi::Object)]
pub struct LocalizationClient {
    container: Arc<Container>,
    environment: NativeblocksEnvironment,
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalizationClient {
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
        ));
        return Ok(Arc::new(Self {
            container,
            environment,
        }));
    }

    pub fn state_manager(&self) -> Arc<LocalizationStateManager> {
        return LocalizationStateManager::new(
            self.container.repository_arc(),
            self.environment.instance_name().to_string(),
        );
    }

    pub async fn sync_localization(&self, language_code: String) -> Result<(), NBError> {
        return self
            .container
            .repository()
            .sync(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub async fn get_localization(
        &self,
        language_code: String,
    ) -> Result<NativeLocalizationModel, NBError> {
        return self
            .container
            .repository()
            .get(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub fn set_language_code(&self, language_code: String) {
        self.container
            .repository()
            .set_language_code(&language_code);
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.container.repository().translate(&key);
    }
}

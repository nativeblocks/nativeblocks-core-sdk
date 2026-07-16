use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::di;
use crate::localization::domain::model::NativeLocalizationModel;
use crate::localization::presenter::state_manager::LocalizationStateManager;

#[derive(uniffi::Object)]
pub struct LocalizationClient {
    services: Arc<di::Services>,
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
        let container = di::get_or_create(&environment, &config)?;
        let services = container.services(http, cache);
        return Ok(Arc::new(Self { services }));
    }

    pub fn state_manager(&self) -> Arc<LocalizationStateManager> {
        return LocalizationStateManager::new(self.services.localization_repository());
    }

    pub async fn sync_localization(&self, language_code: String) -> Result<(), NBError> {
        return self
            .services
            .localization_repository()
            .sync(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub async fn get_localization(
        &self,
        language_code: String,
    ) -> Result<NativeLocalizationModel, NBError> {
        return self
            .services
            .localization_repository()
            .load(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub fn set_language_code(&self, language_code: String) {
        self.services
            .localization_repository()
            .set_language_code(&language_code);
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.services.localization_repository().translate(&key);
    }
}

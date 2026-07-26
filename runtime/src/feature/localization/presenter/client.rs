use std::sync::Arc;

use crate::feature::localization::domain::model::NativeLocalizationModel;
use crate::feature::localization::domain::repository::LocalizationRepository;
use crate::feature::localization::presenter::state_manager::LocalizationStateManager;
use crate::library::result::NBError;

#[derive(uniffi::Object)]
pub struct LocalizationClient {
    repository: Arc<dyn LocalizationRepository>,
}

impl LocalizationClient {
    pub(crate) fn create(repository: Arc<dyn LocalizationRepository>) -> Arc<Self> {
        return Arc::new(Self { repository });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalizationClient {
    pub fn state_manager(&self) -> Arc<LocalizationStateManager> {
        return LocalizationStateManager::new(self.repository.clone());
    }

    pub async fn sync_localization(&self, language_code: String) -> Result<(), NBError> {
        return self
            .repository
            .sync(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub async fn get_localization(
        &self,
        language_code: String,
    ) -> Result<NativeLocalizationModel, NBError> {
        return self
            .repository
            .load(&language_code)
            .await
            .map_err(NBError::from);
    }

    pub fn set_language_code(&self, language_code: String) {
        self.repository.set_language_code(&language_code);
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.repository.translate(&key);
    }
}

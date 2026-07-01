use tokio::sync::watch;

use crate::common::result::NBResult;
use crate::localization::domain::model::NativeLocalizationModel;

#[async_trait::async_trait]
pub(crate) trait LocalizationRepository: Send + Sync {
    async fn sync(&self, language_code: &str) -> NBResult<()>;

    async fn get(&self, language_code: &str) -> NBResult<NativeLocalizationModel>;

    fn set_language_code(&self, language_code: &str);

    fn language_code(&self) -> watch::Receiver<Option<String>>;

    fn translate(&self, key: &str) -> Option<String>;
}

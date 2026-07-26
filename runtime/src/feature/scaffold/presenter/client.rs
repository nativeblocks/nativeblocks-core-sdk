use std::sync::Arc;

use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::feature::scaffold::domain::repository::ScaffoldRepository;
use crate::library::result::NBError;

#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    repository: Arc<dyn ScaffoldRepository>,
}

impl ScaffoldClient {
    pub(crate) fn create(repository: Arc<dyn ScaffoldRepository>) -> Arc<Self> {
        return Arc::new(Self { repository });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ScaffoldClient {
    pub async fn get_scaffold(&self) -> Result<ScaffoldModel, NBError> {
        return self.repository.fetch().await.map_err(NBError::from);
    }
}

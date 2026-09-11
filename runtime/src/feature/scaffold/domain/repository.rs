use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::library::result::NBResult;

#[async_trait::async_trait]
pub(crate) trait ScaffoldRepository: Send + Sync {
    async fn fetch(&self, force_fetch: bool) -> NBResult<ScaffoldModel>;

    async fn clear(&self) -> NBResult<()>;
}

use crate::common::result::NBResult;
use crate::scaffold::domain::model::NativeScaffoldModel;

#[async_trait::async_trait]
pub(crate) trait ScaffoldRepository: Send + Sync {
    async fn fetch(&self) -> NBResult<NativeScaffoldModel>;
}

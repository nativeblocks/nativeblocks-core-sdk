use crate::common::result::NBResult;
use crate::scaffold::data::repository::ScaffoldRepository;
use crate::scaffold::domain::model::{NativeScaffoldModel, ScaffoldRequest};

pub(crate) async fn get_scaffold_use_case(
    repository: &ScaffoldRepository,
    request: &ScaffoldRequest,
) -> NBResult<NativeScaffoldModel> {
    return repository.get_scaffold(request).await;
}

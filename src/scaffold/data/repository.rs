use crate::common::result::NBResult;
use crate::scaffold::data::source::ScaffoldRemoteSource;
use crate::scaffold::domain::model::{NativeScaffoldModel, ScaffoldRequest};

pub(crate) struct ScaffoldRepository {
    source: ScaffoldRemoteSource,
}

impl ScaffoldRepository {
    pub(crate) fn new(source: ScaffoldRemoteSource) -> Self {
        return Self { source };
    }

    pub(crate) async fn get_scaffold(
        &self,
        request: &ScaffoldRequest,
    ) -> NBResult<NativeScaffoldModel> {
        let dto = self.source.fetch(request).await?;
        return Ok(dto.to_model());
    }
}

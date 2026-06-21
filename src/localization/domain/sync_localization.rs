use crate::common::result::NBResult;
use crate::localization::data::repository::LocalizationRepository;
use crate::localization::model::{LocalizationSyncRequest, NativeLocalizationModel};

pub(crate) async fn sync_localization_use_case(
    repository: &LocalizationRepository,
    request: &LocalizationSyncRequest,
) -> NBResult<Option<NativeLocalizationModel>> {
    return repository.sync(request).await;
}

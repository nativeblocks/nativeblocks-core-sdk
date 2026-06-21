use crate::common::result::NBResult;
use crate::localization::data::repository::LocalizationRepository;
use crate::localization::model::NativeLocalizationModel;

pub(crate) async fn get_localization_use_case(
    repository: &LocalizationRepository,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    return repository.load(language_code).await;
}

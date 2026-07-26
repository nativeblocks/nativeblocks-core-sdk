mod data;
mod domain;
mod presenter;

use std::sync::Arc;

use crate::di::Container;
use crate::feature::localization::data::cloud_repository::CloudLocalizationRepository;
use crate::feature::localization::domain::repository::LocalizationRepository;

pub use domain::model::NativeLocalizationModel;
pub use presenter::LocalizationClient;

pub(crate) fn get_or_create_client(container: &Container) -> Arc<LocalizationClient> {
    return container.component(|| LocalizationClient::create(build_repository(container)));
}

fn build_repository(container: &Container) -> Arc<dyn LocalizationRepository> {
    return Arc::new(CloudLocalizationRepository::new(
        container.http(),
        container.environment().clone(),
        container.sdk_config().clone(),
        container.cache(),
        container.config_client(),
        container.logger(),
    ));
}

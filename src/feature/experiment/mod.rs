mod data;
mod domain;
mod presenter;

use std::sync::Arc;

use crate::di::Container;
use crate::feature::experiment::data::repository_impl::ExperimentRepositoryImpl;
use crate::feature::experiment::domain::repository::ExperimentRepository;

pub use domain::model::NativeExperimentModel;
pub use presenter::ExperimentClient;

pub(crate) fn get_or_create_client(container: &Container) -> Arc<ExperimentClient> {
    return container.component(|| {
        ExperimentClient::create(build_repository(container), container.global_parameters())
    });
}

fn build_repository(container: &Container) -> Arc<dyn ExperimentRepository> {
    return Arc::new(ExperimentRepositoryImpl::new(
        container.http(),
        container.environment().clone(),
        container.sdk_config().clone(),
        container.cache(),
        container.config_client(),
        container.logger(),
    ));
}

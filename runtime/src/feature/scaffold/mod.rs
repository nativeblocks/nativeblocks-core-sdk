mod data;
mod domain;
mod presenter;

use std::sync::Arc;

use crate::di::Container;
use crate::feature::scaffold::data::repository_impl::ScaffoldRepositoryImpl;
use crate::feature::scaffold::domain::repository::ScaffoldRepository;

pub use domain::model::{FrameRouteModel, FrameTypeModel, RouteArgumentsModel, ScaffoldModel};
pub use presenter::ScaffoldClient;

pub(crate) fn get_or_create_client(container: &Container) -> Arc<ScaffoldClient> {
    return container.component(|| ScaffoldClient::create(build_repository(container)));
}

fn build_repository(container: &Container) -> Arc<dyn ScaffoldRepository> {
    return ScaffoldRepositoryImpl::new(
        container.http(),
        container.environment().clone(),
        container.sdk_config().clone(),
        container.caches().scaffold.clone(),
        container.config_client(),
        container.logger(),
    );
}

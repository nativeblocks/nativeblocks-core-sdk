#![allow(deprecated)]

mod data;
mod domain;
mod presenter;

use std::sync::Arc;

use crate::di::Container;
use crate::feature::frame::data::cloud_repository::CloudFrameRepository;
use crate::feature::frame::domain::repository::FrameRepository;

pub use domain::model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameModel,
    NativeVariableModel,
};
pub use presenter::FrameClient;

pub(crate) fn get_or_create_client(container: &Container) -> Arc<FrameClient> {
    return container.component(|| {
        FrameClient::create(
            build_repository(container),
            container.global_parameters(),
            container.logger(),
            container.sdk_config().clone(),
        )
    });
}

fn build_repository(container: &Container) -> Arc<dyn FrameRepository> {
    return Arc::new(CloudFrameRepository::new(
        container.http(),
        container.environment().clone(),
        container.sdk_config().clone(),
        container.cache(),
        container.config_client(),
        container.logger(),
    ));
}

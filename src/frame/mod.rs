mod data;
pub(crate) mod di;
mod domain;
mod presenter;

pub(crate) use domain::repository::FrameRepository;

pub use domain::model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameModel,
    NativeVariableModel,
};
pub use presenter::{FrameClient, FrameState, FrameStateManager, FrameStateObserver};
pub(crate) use presenter::dispose;

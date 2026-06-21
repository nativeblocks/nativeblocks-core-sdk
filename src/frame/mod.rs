mod data;
mod di;
mod domain;
mod presenter;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
pub use presenter::ffi::FrameClient;

pub use domain::action::{ActionContext, ActionResult, NativeActionHandler};
pub use domain::model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameState,
    NativeVariableModel,
};

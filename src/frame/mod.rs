pub mod action;
mod client;
pub mod graphql;
pub mod key;
pub mod model;

#[cfg(feature = "script-quickjs")]
mod script;

mod data;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
mod ffi;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
pub use ffi::FrameClient;

pub use action::{ActionContext, ActionResult, NativeActionHandler, SCRIPT_KEY_TYPE};
pub use client::{Client, FrameSyncRequest, new_client};
pub use data::source::FrameLocalSource;

#[cfg(feature = "cache-sqlite")]
pub use client::open_engine;

pub use model::{
    NativeActionModel, NativeActionTriggerDataModel, NativeActionTriggerModel,
    NativeActionTriggerPropertyModel, NativeActionTriggerThen, NativeBlockDataModel,
    NativeBlockModel, NativeBlockPropertyModel, NativeBlockSlotModel, NativeFrameState,
    NativeVariableModel,
};

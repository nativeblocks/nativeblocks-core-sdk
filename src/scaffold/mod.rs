mod data;
mod di;
mod domain;
mod presenter;

pub use domain::model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};
pub use presenter::ffi::ScaffoldClient;

mod client;
mod data;
mod di;
mod domain;
mod ffi;
mod graphql;
mod model;

pub use ffi::ScaffoldClient;
pub use model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

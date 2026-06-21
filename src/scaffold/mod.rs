mod client;
mod ffi;
pub mod graphql;
pub mod model;

mod data;

pub use client::{Client, ScaffoldRequest, new_client};
pub use ffi::ScaffoldClient;
pub use model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

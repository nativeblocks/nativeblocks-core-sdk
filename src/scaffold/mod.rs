mod client;
pub mod graphql;
pub mod model;

mod data;

pub use client::{Client, ScaffoldRequest, new_client};
pub use model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

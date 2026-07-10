mod data;
pub(crate) mod di;
mod domain;
mod presenter;

pub use domain::model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};
pub use presenter::ScaffoldClient;

pub(crate) use domain::repository::ScaffoldRepository;

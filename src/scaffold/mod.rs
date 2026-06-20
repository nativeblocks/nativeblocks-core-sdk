//! Scaffold feature: fetch a project's scaffold (route table).
//!
//! Mirrors the Go feature-package pattern (`exm/auth/`): the feature owns its
//! full vertical (model → dto → mapper → fetch → client) and exposes itself
//! **only** through [`Client`]. Other features depend on it via [`new_client`]
//! and [`Client`] alone — never its internals.
//!
//! Ports the Kotlin `ScaffoldRepository` slice of `frame/`.

mod client;
pub mod key;
pub mod model;

mod data;

pub use client::{Client, ScaffoldRequest, new_client};
pub use model::{
    FrameTypeModel, NativeFrameRouteModel, NativeRouteArgumentsModel, NativeScaffoldModel,
};

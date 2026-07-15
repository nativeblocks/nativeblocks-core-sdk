mod client;
mod logging;
mod state_manager;

pub use client::FrameClient;
pub use state_manager::{FrameState, FrameStateManager, FrameStateObserver};

pub(crate) fn dispose(_instance_name: &str) {}

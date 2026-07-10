mod action_context;
mod block_context;
mod client;
mod logging;
mod script_runner;
mod state_manager;

pub use action_context::{ActionContext, HostActionDispatcher};
pub use block_context::{BlockObserver, BlockContext};
pub use client::FrameClient;
pub use state_manager::{FrameState, FrameStateManager, FrameStateObserver};

pub(crate) fn dispose(_instance_name: &str) {}

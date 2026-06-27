mod action_props;
mod action_provider;
mod action_tree;
mod block_props;
mod block_provider;
mod block_tree;
mod client;
mod state_manager;

pub use client::FrameClient;
pub use state_manager::{FrameStateObserver, FrameState, FrameStateManager};

pub(crate) fn dispose(instance_name: &str) {
    action_provider::remove(instance_name);
    block_provider::remove(instance_name);
}

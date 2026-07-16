mod client;
mod state_manager;

pub use client::LocalizationClient;
pub use state_manager::{LocalizationState, LocalizationStateManager, LocalizationStateObserver};

pub(crate) fn dispose(_instance_name: &str) {}

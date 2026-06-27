mod client;
mod provider;

pub use client::GlobalParameterClient;
pub(crate) use provider::{get_or_create, remove};

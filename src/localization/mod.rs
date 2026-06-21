mod data;
mod di;
mod domain;
mod presenter;

pub(crate) use presenter::client::Client;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
pub use presenter::ffi::LocalizationClient;

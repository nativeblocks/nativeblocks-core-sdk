mod client;
pub mod graphql;
pub mod key;

mod data;
mod model;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
mod ffi;

pub use client::{Client, LocalizationSyncRequest, new_client};
pub use data::source::LocalizationLocalSource;

#[cfg(all(feature = "net-reqwest", feature = "cache-sqlite"))]
pub use ffi::LocalizationClient;

#[cfg(feature = "cache-sqlite")]
pub use client::open_client;

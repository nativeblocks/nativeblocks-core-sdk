// These clippy lints flag deliberate, codebase-wide patterns rather than defects:
#![allow(clippy::needless_return)] // explicit `return` is the house style
#![allow(clippy::too_many_arguments)] // data-layer fns thread request context explicitly
#![allow(clippy::collapsible_if)] // nested `if let` kept; let-chains are still unstable
#![allow(clippy::match_single_binding)] // versioned cache-migration dispatch placeholder
#![allow(clippy::wildcard_in_or_patterns)] // explicit default branch documents intent

uniffi::setup_scaffolding!("NativeblocksCoreEngine");

pub mod common;
pub mod config;
// pub mod experiment; // disabled for now — not needed yet
pub mod frame;
pub mod global_parameter;
pub mod instance;
pub mod localization;
pub mod scaffold;
#[cfg(feature = "script-quickjs")]
pub mod script;

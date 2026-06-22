pub mod keys;

mod provider;

use std::collections::HashMap;

pub use provider::{NativeLoggerProvider, get_or_create, remove};

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum LoggerEventLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[uniffi::export(callback_interface)]
pub trait Logger: Send + Sync {
    /// `level` — severity.
    /// `event` — event state/type (see [`keys::state`] / [`keys::tag`]).
    /// `message` — human-readable message.
    /// `parameters` — event-specific key/value data (see [`keys::parameter`]).
    fn log(
        &self,
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: HashMap<String, String>,
    );
}

#[uniffi::export]
pub fn provide_logger(instance_name: String, logger_type: String, logger: Box<dyn Logger>) {
    if let Ok(mut provider) = get_or_create(&instance_name).lock() {
        provider.provide_logger(logger_type, logger);
    }
}

#[uniffi::export]
pub fn remove_logger(instance_name: String, logger_type: String) {
    if let Ok(mut provider) = get_or_create(&instance_name).lock() {
        provider.remove_logger(&logger_type);
    }
}

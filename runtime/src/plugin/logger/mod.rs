pub mod keys;

mod provider;

use std::collections::HashMap;

use crate::library::result::ErrorModel;

pub use provider::{NativeLoggerProvider, get_or_create, remove};

pub(crate) fn error_parameters(error: &ErrorModel) -> HashMap<String, String> {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::ERROR_MESSAGE.to_string(),
        error.message.clone(),
    );
    params.insert(
        keys::parameter::ERROR_TYPE.to_string(),
        error.error_type.as_str().to_string(),
    );
    if let Some(code) = &error.error_code {
        params.insert(keys::parameter::ERROR_TAG.to_string(), code.clone());
    }
    return params;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum LoggerEventLevel {
    Debug,
    Error,
}

#[uniffi::export(callback_interface)]
pub trait Logger: Send + Sync {
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

pub mod keys;

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::common::config::SdkConfig;

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
pub fn provide_event_logger(instance_name: String, logger_type: String, logger: Box<dyn Logger>) {
    if let Ok(mut provider) = get_or_create(&instance_name).lock() {
        provider.provide_event_logger(logger_type, logger);
    }
}

#[uniffi::export]
pub fn remove_event_logger(instance_name: String, logger_type: String) {
    if let Ok(mut provider) = get_or_create(&instance_name).lock() {
        provider.remove_event_logger(&logger_type);
    }
}

pub fn log_with_context(
    logger: &dyn Logger,
    config: &SdkConfig,
    level: LoggerEventLevel,
    event: impl Into<String>,
    message: impl Into<String>,
    mut parameters: HashMap<String, String>,
) {
    parameters.insert("SDK-Version".to_string(), config.version.clone());
    parameters.insert("SDK-Platform".to_string(), config.platform.clone());
    logger.log(level, event.into(), message.into(), parameters);
}

#[derive(Default)]
pub struct NativeLoggerProvider {
    loggers: HashMap<String, Box<dyn Logger>>,
}

impl NativeLoggerProvider {
    pub fn provide_event_logger(
        &mut self,
        logger_type: impl Into<String>,
        logger: Box<dyn Logger>,
    ) {
        self.loggers.insert(logger_type.into(), logger);
    }

    pub fn remove_event_logger(&mut self, logger_type: &str) {
        self.loggers.remove(logger_type);
    }

    pub fn logger_types(&self) -> Vec<String> {
        self.loggers.keys().cloned().collect()
    }

    pub fn dispatch(
        &self,
        config: &SdkConfig,
        level: LoggerEventLevel,
        event: impl Into<String>,
        message: impl Into<String>,
        parameters: HashMap<String, String>,
    ) {
        let event = event.into();
        let message = message.into();
        for logger in self.loggers.values() {
            log_with_context(
                logger.as_ref(),
                config,
                level,
                event.clone(),
                message.clone(),
                parameters.clone(),
            );
        }
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<Mutex<NativeLoggerProvider>>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Mutex<NativeLoggerProvider>>>>> =
        OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_or_create(instance_name: &str) -> Arc<Mutex<NativeLoggerProvider>> {
    let mut map = registry().lock().expect("logger registry poisoned");
    map.entry(instance_name.to_string())
        .or_insert_with(|| Arc::new(Mutex::new(NativeLoggerProvider::default())))
        .clone()
}

pub fn remove(instance_name: &str) {
    registry()
        .lock()
        .expect("logger registry poisoned")
        .remove(instance_name);
}

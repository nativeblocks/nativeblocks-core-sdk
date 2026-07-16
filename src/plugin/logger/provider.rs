use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::library::environment::model::SdkConfig;

use super::{Logger, LoggerEventLevel};

const SDK_VERSION_PARAM: &str = "SDK-Version";
const SDK_PLATFORM_PARAM: &str = "SDK-Platform";

fn log_with_context(
    logger: &dyn Logger,
    config: &SdkConfig,
    level: LoggerEventLevel,
    event: impl Into<String>,
    message: impl Into<String>,
    mut parameters: HashMap<String, String>,
) {
    parameters.insert(SDK_VERSION_PARAM.to_string(), config.version.clone());
    parameters.insert(SDK_PLATFORM_PARAM.to_string(), config.platform.clone());
    logger.log(level, event.into(), message.into(), parameters);
}

#[derive(Default)]
pub struct NativeLoggerProvider {
    loggers: HashMap<String, Box<dyn Logger>>,
}

impl NativeLoggerProvider {
    pub fn provide_logger(&mut self, logger_type: impl Into<String>, logger: Box<dyn Logger>) {
        self.loggers.insert(logger_type.into(), logger);
    }

    pub fn remove_logger(&mut self, logger_type: &str) {
        self.loggers.remove(logger_type);
    }

    pub fn logger_types(&self) -> Vec<String> {
        self.loggers.keys().cloned().collect()
    }

    pub fn has_loggers(&self) -> bool {
        return !self.loggers.is_empty();
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

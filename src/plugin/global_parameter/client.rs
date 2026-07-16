use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::library::environment::model::SdkConfig;
use crate::plugin::global_parameter::GlobalParameterProvider;
use crate::plugin::logger::{keys, LoggerEventLevel, NativeLoggerProvider};

#[derive(uniffi::Object)]
pub struct GlobalParameterClient {
    globals: Arc<GlobalParameterProvider>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    sdk_config: SdkConfig,
}

impl GlobalParameterClient {
    pub(crate) fn create(
        globals: Arc<GlobalParameterProvider>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        sdk_config: SdkConfig,
    ) -> Arc<Self> {
        return Arc::new(Self {
            globals,
            logger,
            sdk_config,
        });
    }
}

#[uniffi::export]
impl GlobalParameterClient {
    pub fn set(&self, parameters: HashMap<String, String>) {
        self.globals.set(parameters.clone());
        self.log(LoggerEventLevel::Info, "Global parameters set", parameters);
    }

    pub fn get(&self) -> HashMap<String, String> {
        return self.globals.get();
    }
}

impl GlobalParameterClient {
    fn log(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(logger_provider) = self.logger.lock() {
            logger_provider.dispatch(
                &self.sdk_config,
                level,
                keys::tag::GLOBAL_PARAMETERS_CHANGE,
                message,
                params,
            );
        }
    }
}

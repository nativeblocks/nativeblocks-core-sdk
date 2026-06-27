use std::collections::HashMap;
use std::sync::Arc;

use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, keys};
use crate::common::result::NBError;
use crate::global_parameter::provider;

/// Public entry point for the global-parameter feature. Global parameters are
/// instance-wide host-set key/values consumed by other features (e.g. merged
/// into a frame's variables and sent to the backend on sync).
#[derive(uniffi::Object)]
pub struct GlobalParameterClient {
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
}

#[uniffi::export]
impl GlobalParameterClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Result<Arc<Self>, NBError> {
        environment.validate()?;
        return Ok(Arc::new(Self {
            environment,
            sdk_config: config,
        }));
    }

    /// Replace the instance-wide global parameters. Any edition restriction is a
    /// host-SDK concern; the engine just stores them.
    pub fn set(&self, parameters: HashMap<String, String>) {
        provider::get_or_create(self.environment.instance_name()).set(parameters.clone());
        self.log(LoggerEventLevel::Info, "Global parameters set", parameters);
    }

    pub fn get(&self) -> HashMap<String, String> {
        return provider::get_or_create(self.environment.instance_name()).get();
    }
}

impl GlobalParameterClient {
    fn log(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(logger_provider) = logger::get_or_create(self.environment.instance_name()).lock() {
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

use std::collections::HashMap;
use std::sync::Arc;

use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{keys, LoggerEventLevel};
use crate::common::result::NBError;
use crate::di;

/// Public entry point for the global-parameter feature. Global parameters are
/// instance-wide host-set key/values consumed by other features (e.g. merged
/// into a frame's variables and sent to the backend on sync). This client
/// needs no IO dependencies, so it resolves only the container's core layer.
#[derive(uniffi::Object)]
pub struct GlobalParameterClient {
    container: Arc<di::Container>,
}

#[uniffi::export]
impl GlobalParameterClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Result<Arc<Self>, NBError> {
        let container = di::get_or_create(&environment, &config)?;
        return Ok(Arc::new(Self { container }));
    }

    /// Replace the instance-wide global parameters. Any edition restriction is a
    /// host-SDK concern; the engine just stores them.
    pub fn set(&self, parameters: HashMap<String, String>) {
        self.container.global_parameters().set(parameters.clone());
        self.log(LoggerEventLevel::Info, "Global parameters set", parameters);
    }

    pub fn get(&self) -> HashMap<String, String> {
        return self.container.global_parameters().get();
    }
}

impl GlobalParameterClient {
    fn log(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(logger_provider) = self.container.logger().lock() {
            logger_provider.dispatch(
                self.container.sdk_config(),
                level,
                keys::tag::GLOBAL_PARAMETERS_CHANGE,
                message,
                params,
            );
        }
    }
}

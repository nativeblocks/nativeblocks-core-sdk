use std::collections::HashMap;
use std::sync::Mutex;

use crate::library::environment::model::SdkConfig;
use crate::library::result::ErrorModel;
use crate::plugin::logger::{keys, LoggerEventLevel, NativeLoggerProvider};

pub(super) fn log_success(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    frames_count: usize,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::SCAFFOLD_FETCH_SUCCEED.to_string(),
    );
    params.insert(
        keys::parameter::FRAMES_COUNT.to_string(),
        frames_count.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        "Successfully fetched scaffold",
        params,
    );
}

pub(super) fn log_failure(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    error: &ErrorModel,
) {
    let mut params = crate::plugin::logger::error_parameters(error);
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::SCAFFOLD_FETCH_FAILED.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        "Failed to fetch scaffold",
        params,
    );
}

fn dispatch(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    level: LoggerEventLevel,
    message: &str,
    params: HashMap<String, String>,
) {
    if let Ok(provider) = logger.lock() {
        provider.dispatch(
            sdk_config,
            level,
            keys::tag::SCAFFOLD_FETCH,
            message,
            params,
        );
    }
}

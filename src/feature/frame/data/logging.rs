use std::collections::HashMap;
use std::sync::Mutex;

use crate::library::environment::model::SdkConfig;
use crate::library::result::ErrorModel;
use crate::plugin::logger::{keys, LoggerEventLevel, NativeLoggerProvider};

pub(super) fn log_sync_success(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    route: &str,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::FRAME_SYNC_SUCCEED.to_string(),
    );
    params.insert(keys::parameter::FRAME_ROUTE.to_string(), route.to_string());
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        "Successfully synced frame",
        params,
    );
}

pub(super) fn log_sync_failure(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    route: &str,
    error: &ErrorModel,
) {
    let mut params = crate::plugin::logger::error_parameters(error);
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::FRAME_SYNC_FAILED.to_string(),
    );
    params.insert(keys::parameter::FRAME_ROUTE.to_string(), route.to_string());
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        "Failed to sync frame",
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
            keys::tag::FRAME_SYNC_STATE,
            message,
            params,
        );
    }
}

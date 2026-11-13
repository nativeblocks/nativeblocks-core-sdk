use std::collections::HashMap;
use std::sync::Mutex;

use crate::library::environment::model::SdkConfig;
use crate::library::result::ErrorModel;
use crate::plugin::logger::{LoggerEventLevel, NativeLoggerProvider, keys};

pub(super) fn log_seeded(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    frames_count: usize,
    localizations_count: usize,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::EMBEDDED_SEEDED.to_string(),
    );
    params.insert(
        keys::parameter::FRAMES_COUNT.to_string(),
        frames_count.to_string(),
    );
    params.insert(
        keys::parameter::LOCALIZATIONS_COUNT.to_string(),
        localizations_count.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Debug,
        "Copied embedded content into the cache",
        params,
    );
}

pub(super) fn log_skipped(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    key: &str,
    reason: &str,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::EMBEDDED_ENTRY_SKIPPED.to_string(),
    );
    params.insert(keys::parameter::KEY.to_string(), key.to_string());
    params.insert(
        keys::parameter::ERROR_MESSAGE.to_string(),
        reason.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        "Skipped embedded content",
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
        keys::state::EMBEDDED_SEED_FAILED.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        "Failed to copy embedded content into the cache",
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
        provider.dispatch(sdk_config, level, keys::tag::EMBEDDED_SEED, message, params);
    }
}

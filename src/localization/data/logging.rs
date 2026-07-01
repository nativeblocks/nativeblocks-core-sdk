use std::collections::HashMap;
use std::sync::Mutex;

use crate::common::environment::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::ErrorModel;

pub(super) fn log_sync_success(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    language_code: &str,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::LOCALIZATION_SYNC_SUCCEED.to_string(),
    );
    params.insert(
        keys::parameter::LANGUAGE_CODE.to_string(),
        language_code.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        keys::tag::LOCALIZATION_SYNC_STATE,
        "Localization synced",
        params,
    );
}

pub(super) fn log_sync_failure(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    language_code: &str,
    error: &ErrorModel,
) {
    let mut params = error.to_logger_parameters();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::LOCALIZATION_SYNC_FAILED.to_string(),
    );
    params.insert(
        keys::parameter::LANGUAGE_CODE.to_string(),
        language_code.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        keys::tag::LOCALIZATION_SYNC_STATE,
        "Failed to sync localization",
        params,
    );
}

pub(super) fn log_load_success(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    language_code: &str,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::LOCALIZATION_LOAD_SUCCEED.to_string(),
    );
    params.insert(
        keys::parameter::LANGUAGE_CODE.to_string(),
        language_code.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        keys::tag::LOCALIZATION_STATE,
        "Localization loaded",
        params,
    );
}

pub(super) fn log_load_failure(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    language_code: &str,
    error: &ErrorModel,
) {
    let mut params = error.to_logger_parameters();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::LOCALIZATION_LOAD_FAILED.to_string(),
    );
    params.insert(
        keys::parameter::LANGUAGE_CODE.to_string(),
        language_code.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        keys::tag::LOCALIZATION_STATE,
        "Failed to load localization",
        params,
    );
}

pub(super) fn log_language_set(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    language_code: &str,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::LOCALIZATION_SET.to_string(),
    );
    params.insert(
        keys::parameter::LANGUAGE_CODE.to_string(),
        language_code.to_string(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        keys::tag::LOCALIZATION_STATE,
        "Language changed",
        params,
    );
}

fn dispatch(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    level: LoggerEventLevel,
    tag: &str,
    message: &str,
    params: HashMap<String, String>,
) {
    if let Ok(provider) = logger.lock() {
        provider.dispatch(sdk_config, level, tag, message, params);
    }
}

use std::collections::HashMap;
use std::sync::Mutex;

use crate::feature::experiment::domain::model::NativeExperimentModel;
use crate::library::environment::model::SdkConfig;
use crate::library::result::ErrorModel;
use crate::plugin::logger::{keys, LoggerEventLevel, NativeLoggerProvider};

pub(super) fn log_success(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    key: &str,
    experiment: &NativeExperimentModel,
) {
    let mut params = HashMap::new();
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::EXPERIMENT_FETCH_SUCCEED.to_string(),
    );
    params.insert(keys::parameter::EXPERIMENT_KEY.to_string(), key.to_string());
    params.insert(
        keys::parameter::EXPERIMENT_VALUE.to_string(),
        experiment.value.clone(),
    );
    params.insert(
        keys::parameter::EXPERIMENT_TYPE.to_string(),
        experiment.variable_type.clone(),
    );
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Info,
        &format!("Get experiment with key: {key}"),
        params,
    );
}

pub(super) fn log_failure(
    logger: &Mutex<NativeLoggerProvider>,
    sdk_config: &SdkConfig,
    key: &str,
    error: &ErrorModel,
) {
    let mut params = crate::plugin::logger::error_parameters(error);
    params.insert(
        keys::parameter::STATE.to_string(),
        keys::state::EXPERIMENT_FETCH_FAILED.to_string(),
    );
    params.insert(keys::parameter::EXPERIMENT_KEY.to_string(), key.to_string());
    dispatch(
        logger,
        sdk_config,
        LoggerEventLevel::Error,
        &format!("Get experiment failed for experiment: {key}"),
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
            keys::tag::EXPERIMENT_STATE,
            message,
            params,
        );
    }
}

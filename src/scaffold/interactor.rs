use std::collections::HashMap;
use std::sync::Mutex;

use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::net::HttpClient;
use crate::common::result::{ErrorModel, NBResult};
use crate::config;
use crate::scaffold::key::GATEWAY_OPERATION;
use crate::scaffold::model::NativeScaffoldModel;
use crate::scaffold::repository::fetch_scaffold;

pub(super) async fn get_use_case(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    config_client: &config::Client,
) -> NBResult<NativeScaffoldModel> {
    let resolved = config_client.gateway(GATEWAY_OPERATION).await?;
    return fetch_scaffold(
        http,
        environment,
        sdk_config,
        resolved.gateway,
        &resolved.endpoint,
        &resolved.install_id,
    )
    .await;
}

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
    let mut params = error.to_logger_parameters();
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

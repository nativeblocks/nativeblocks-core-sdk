use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::net::HttpClient;
use crate::common::result::{ErrorModel, NbError};
use crate::config;
use crate::scaffold::key::GATEWAY_OPERATION;
use crate::scaffold::model::NativeScaffoldModel;
use crate::scaffold::source::fetch_scaffold;

#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    http: Arc<dyn HttpClient>,
    config_client: Arc<config::Client>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ScaffoldClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NbError> {
        environment.validate()?;
        let config_client = config::get_or_create(http.clone(), &environment, &config, cache);
        let logger = logger::get_or_create(environment.instance_name());
        return Ok(Arc::new(Self {
            http,
            config_client,
            environment,
            sdk_config: config,
            logger,
        }));
    }

    pub async fn get_scaffold(&self) -> Result<NativeScaffoldModel, NbError> {
        let resolved = self.config_client.gateway(GATEWAY_OPERATION).await?;
        let result = fetch_scaffold(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            resolved.gateway,
            &resolved.endpoint,
            &resolved.install_id,
        )
        .await;
        match &result {
            Ok(scaffold) => self.log_success(scaffold.frames.len()),
            Err(error) => self.log_failure(error),
        }
        return result.map_err(NbError::from);
    }
}

impl ScaffoldClient {
    fn log_success(&self, frames_count: usize) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_SUCCEED.to_string(),
        );
        params.insert(
            keys::parameter::FRAMES_COUNT.to_string(),
            frames_count.to_string(),
        );
        self.dispatch(LoggerEventLevel::Info, "Successfully fetched scaffold", params);
    }

    fn log_failure(&self, error: &ErrorModel) {
        let mut params = error.to_logger_parameters();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_FAILED.to_string(),
        );
        self.dispatch(LoggerEventLevel::Error, "Failed to fetch scaffold", params);
    }

    fn dispatch(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(
                &self.sdk_config,
                level,
                keys::tag::SCAFFOLD_FETCH,
                message,
                params,
            );
        }
    }
}

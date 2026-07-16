use std::sync::{Arc, Mutex};

use crate::feature::scaffold::data::key::GATEWAY_OPERATION;
use crate::feature::scaffold::data::logging;
use crate::feature::scaffold::data::source;
use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::feature::scaffold::domain::repository::ScaffoldRepository;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;
use crate::plugin::config;
use crate::plugin::logger::NativeLoggerProvider;

pub(crate) struct ScaffoldRepositoryImpl {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

impl ScaffoldRepositoryImpl {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        config_client: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            config_client,
            logger,
        };
    }

    async fn fetch_scaffold(&self) -> NBResult<ScaffoldModel> {
        let resolved = self.config_client.gateway(GATEWAY_OPERATION).await?;
        return source::fetch_scaffold(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            resolved.gateway,
            &resolved.endpoint,
            &resolved.install_id,
        )
        .await;
    }
}

#[async_trait::async_trait]
impl ScaffoldRepository for ScaffoldRepositoryImpl {
    async fn fetch(&self) -> NBResult<ScaffoldModel> {
        let result = self.fetch_scaffold().await;
        match &result {
            Ok(scaffold) => {
                logging::log_success(&self.logger, &self.sdk_config, scaffold.frames.len())
            }
            Err(error) => logging::log_failure(&self.logger, &self.sdk_config, error),
        }
        return result;
    }
}

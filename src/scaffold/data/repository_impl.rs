use std::sync::{Arc, Mutex};

use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::scaffold::data::key::GATEWAY_OPERATION;
use crate::scaffold::data::logging;
use crate::scaffold::data::source;
use crate::scaffold::domain::model::NativeScaffoldModel;
use crate::scaffold::domain::repository::ScaffoldRepository;

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

    async fn fetch_scaffold(&self) -> NBResult<NativeScaffoldModel> {
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
    async fn fetch(&self) -> NBResult<NativeScaffoldModel> {
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

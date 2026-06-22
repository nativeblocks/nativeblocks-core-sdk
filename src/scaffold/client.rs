use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{self, NativeLoggerProvider};
use crate::common::net::HttpClient;
use crate::common::result::NbError;
use crate::config;
use crate::scaffold::interactor;
use crate::scaffold::model::NativeScaffoldModel;

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
        let config_client = config::get_or_create(http.clone(), &environment, &config, cache)?;
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
        let result = interactor::get_use_case(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            self.config_client.as_ref(),
        )
        .await;
        match &result {
            Ok(scaffold) => {
                interactor::log_success(self.logger.as_ref(), &self.sdk_config, scaffold.frames.len())
            }
            Err(error) => interactor::log_failure(self.logger.as_ref(), &self.sdk_config, error),
        }
        return result.map_err(NbError::from);
    }
}

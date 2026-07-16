use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::di;
use crate::experiment::domain::model::NativeExperimentModel;

#[derive(uniffi::Object)]
pub struct ExperimentClient {
    container: Arc<di::Container>,
    services: Arc<di::Services>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ExperimentClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NBError> {
        let container = di::get_or_create(&environment, &config)?;
        let services = container.services(http, cache);
        return Ok(Arc::new(Self {
            container,
            services,
        }));
    }

    pub async fn get_experiment(
        &self,
        key: String,
        cache_ttl: Option<i64>,
    ) -> Result<NativeExperimentModel, NBError> {
        let globals = self.container.global_parameters().get();
        return self
            .services
            .experiment_repository()
            .fetch(&key, cache_ttl, &globals)
            .await
            .map_err(NBError::from);
    }
}

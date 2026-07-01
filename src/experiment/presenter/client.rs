use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBError;
use crate::experiment::di::container::Container;
use crate::experiment::domain::model::NativeExperimentModel;
use crate::global_parameter;

#[derive(uniffi::Object)]
pub struct ExperimentClient {
    container: Arc<Container>,
    environment: NativeblocksEnvironment,
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
        environment.validate()?;
        let container = Arc::new(Container::new(environment.clone(), config, http, cache));
        return Ok(Arc::new(Self {
            container,
            environment,
        }));
    }

    pub async fn get_experiment(
        &self,
        key: String,
        cache_ttl: Option<i64>,
    ) -> Result<NativeExperimentModel, NBError> {
        let globals = global_parameter::get_or_create(self.environment.instance_name()).get();
        return self
            .container
            .repository()
            .fetch(&key, cache_ttl, &globals)
            .await
            .map_err(NBError::from);
    }
}

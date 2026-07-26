use std::sync::Arc;

use crate::feature::experiment::domain::model::NativeExperimentModel;
use crate::feature::experiment::domain::repository::ExperimentRepository;
use crate::library::result::NBError;
use crate::plugin::global_parameter::GlobalParameterProvider;

#[derive(uniffi::Object)]
pub struct ExperimentClient {
    repository: Arc<dyn ExperimentRepository>,
    globals: Arc<GlobalParameterProvider>,
}

impl ExperimentClient {
    pub(crate) fn create(
        repository: Arc<dyn ExperimentRepository>,
        globals: Arc<GlobalParameterProvider>,
    ) -> Arc<Self> {
        return Arc::new(Self {
            repository,
            globals,
        });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ExperimentClient {
    pub async fn get_experiment(
        &self,
        key: String,
        cache_ttl: Option<i64>,
    ) -> Result<NativeExperimentModel, NBError> {
        let globals = self.globals.get();
        return self
            .repository
            .fetch(&key, cache_ttl, &globals)
            .await
            .map_err(NBError::from);
    }
}

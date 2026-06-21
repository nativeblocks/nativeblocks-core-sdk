use std::collections::HashMap;
use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::net::reqwest_client::ReqwestHttpClient;
use crate::common::result::NbError;
use crate::config;
use crate::experiment::client::{self, Client, ExperimentRequest};
use crate::experiment::graphql::GATEWAY_OPERATION;
use crate::experiment::model::NativeExperimentModel;

/// UniFFI handle for the experiment feature. Wraps the internal
/// `experiment::Client` and projects its surface across the FFI boundary. The
/// real GraphQL endpoint and the experiment gateway are resolved from the
/// project config (`config::Client`).
#[derive(uniffi::Object)]
pub struct ExperimentClient {
    inner: Arc<dyn Client>,
    config: Arc<dyn config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ExperimentClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        db_path: String,
    ) -> Result<Arc<Self>, NbError> {
        let http: Arc<dyn HttpClient> = Arc::new(ReqwestHttpClient::new()?);
        let config_client = config::get_or_create(http.clone(), &environment, &config);
        let inner = client::open_client(http, environment, config, &db_path)?;
        Ok(Arc::new(Self {
            inner,
            config: config_client,
        }))
    }

    pub async fn get_experiment(
        &self,
        install_id: String,
        key: String,
        parameters: HashMap<String, String>,
        cache_ttl_millis: i64,
    ) -> Result<NativeExperimentModel, NbError> {
        let project = self.config.project_config(&install_id).await?;
        let request = ExperimentRequest {
            gateway: config::gateway_for(&project, GATEWAY_OPERATION),
            graphql_endpoint: project.endpoint.clone(),
            install_id,
            key,
            parameters,
            cache_ttl_millis,
        };
        self.inner
            .get_experiment(request)
            .await
            .map_err(NbError::from)
    }
}

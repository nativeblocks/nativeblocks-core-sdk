use std::collections::HashMap;
use std::sync::Arc;

use crate::common::cache::new_cache_provider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::new_http_client;
use crate::common::result::NbError;
use crate::config;
use crate::experiment::presenter::client::Client;
use crate::experiment::di;
use crate::experiment::data::graphql::GATEWAY_OPERATION;
use crate::experiment::domain::model::{ExperimentRequest, NativeExperimentModel};

/// UniFFI handle for the experiment feature. Wraps the internal experiment
/// `Client` and projects its surface across the FFI boundary. The real GraphQL
/// endpoint, the experiment gateway and the install id are resolved from the
/// project config (`config::Client`).
#[derive(uniffi::Object)]
pub struct ExperimentClient {
    inner: Client,
    config: Arc<config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ExperimentClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        db_path: String,
    ) -> Result<Arc<Self>, NbError> {
        environment.validate()?;
        let http = new_http_client()?;
        let cache = new_cache_provider(&db_path)?;
        let config_client =
            config::get_or_create(http.clone(), &environment, &config, cache.clone());
        let inner = di::new_client(http, environment, config, cache);
        return Ok(Arc::new(Self {
            inner,
            config: config_client,
        }));
    }

    pub async fn get_experiment(
        &self,
        key: String,
        parameters: HashMap<String, String>,
        cache_ttl_millis: i64,
    ) -> Result<NativeExperimentModel, NbError> {
        let resolved = self.config.gateway(GATEWAY_OPERATION).await?;
        let request = ExperimentRequest {
            gateway: resolved.gateway,
            graphql_endpoint: resolved.endpoint,
            install_id: resolved.install_id,
            key,
            parameters,
            cache_ttl_millis,
        };
        return self
            .inner
            .get_experiment(request)
            .await
            .map_err(NbError::from);
    }
}

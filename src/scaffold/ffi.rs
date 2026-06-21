use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::net::reqwest_client::ReqwestHttpClient;
use crate::common::result::NbError;
use crate::config;
use crate::scaffold::client::{self, Client, ScaffoldRequest};
use crate::scaffold::graphql::GATEWAY_OPERATION;
use crate::scaffold::model::NativeScaffoldModel;

/// UniFFI handle for the scaffold feature. Wraps the internal
/// `scaffold::Client` and projects its surface across the FFI boundary. The
/// real GraphQL endpoint and the scaffold gateway are resolved from the project
/// config (`config::Client`); the host only supplies the install id.
#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    inner: Arc<dyn Client>,
    config: Arc<dyn config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ScaffoldClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Result<Arc<Self>, NbError> {
        let http: Arc<dyn HttpClient> = Arc::new(ReqwestHttpClient::new()?);
        let config_client = config::get_or_create(http.clone(), &environment, &config);
        let inner = client::new_client(http, environment, config);
        Ok(Arc::new(Self {
            inner,
            config: config_client,
        }))
    }

    pub async fn get_scaffold(&self, install_id: String) -> Result<NativeScaffoldModel, NbError> {
        let project = self.config.project_config(&install_id).await?;
        let request = ScaffoldRequest {
            gateway: config::gateway_for(&project, GATEWAY_OPERATION),
            graphql_endpoint: project.endpoint.clone(),
            install_id,
        };
        self.inner
            .get_scaffold(request)
            .await
            .map_err(NbError::from)
    }
}

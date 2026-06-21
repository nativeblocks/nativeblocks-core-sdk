use std::sync::Arc;

use crate::common::cache::new_cache_provider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::new_http_client;
use crate::common::result::NbError;
use crate::config;
use crate::scaffold::presenter::client::Client;
use crate::scaffold::di;
use crate::scaffold::data::graphql::GATEWAY_OPERATION;
use crate::scaffold::domain::model::{NativeScaffoldModel, ScaffoldRequest};

/// UniFFI handle for the scaffold feature. Wraps the internal scaffold `Client`
/// and projects its surface across the FFI boundary. The real GraphQL endpoint,
/// the scaffold gateway and the install id are resolved from the project config
/// (`config::Client`).
#[derive(uniffi::Object)]
pub struct ScaffoldClient {
    inner: Client,
    config: Arc<config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl ScaffoldClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        db_path: String,
    ) -> Result<Arc<Self>, NbError> {
        environment.validate()?;
        let http = new_http_client()?;
        let cache = new_cache_provider(&db_path)?;
        let config_client = config::get_or_create(http.clone(), &environment, &config, cache);
        let inner = di::new_client(http, environment, config);
        return Ok(Arc::new(Self {
            inner,
            config: config_client,
        }));
    }

    pub async fn get_scaffold(&self) -> Result<NativeScaffoldModel, NbError> {
        let resolved = self.config.gateway(GATEWAY_OPERATION).await?;
        let request = ScaffoldRequest {
            gateway: resolved.gateway,
            graphql_endpoint: resolved.endpoint,
            install_id: resolved.install_id,
        };
        return self.inner.get_scaffold(request).await.map_err(NbError::from);
    }
}

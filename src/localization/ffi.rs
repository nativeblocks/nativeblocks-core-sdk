use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::net::reqwest_client::ReqwestHttpClient;
use crate::common::result::NbError;
use crate::config;
use crate::localization::client::{self, Client, LocalizationSyncRequest};
use crate::localization::key::operation;

/// UniFFI handle for the localization feature. Wraps the internal
/// `localization::Client` and projects its surface across the FFI boundary. The
/// real GraphQL endpoint and the localization gateways are resolved from the
/// project config (`config::Client`).
#[derive(uniffi::Object)]
pub struct LocalizationClient {
    inner: Arc<dyn Client>,
    config: Arc<dyn config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalizationClient {
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

    pub async fn sync_localization(
        &self,
        install_id: String,
        language_code: String,
    ) -> Result<(), NbError> {
        let project = self.config.project_config(&install_id).await?;
        let request = LocalizationSyncRequest {
            endpoint_localization: config::gateway_for(&project, operation::LOCALIZATIONS),
            endpoint_localization_production: config::gateway_for(
                &project,
                operation::LOCALIZATIONS_PRODUCTION,
            ),
            endpoint_localization_production_checksum: config::gateway_for(
                &project,
                operation::PRODUCTION_CHECKSUM,
            ),
            graphql_endpoint: project.endpoint.clone(),
            language_code,
            install_id,
        };
        self.inner
            .sync_localization(request)
            .await
            .map_err(NbError::from)
    }

    pub async fn load_localization(&self, language_code: String) -> Result<(), NbError> {
        self.inner
            .get_localization(language_code)
            .await
            .map_err(NbError::from)
    }

    pub fn set_language_code(&self, language_code: String) {
        self.inner.set_language_code(language_code);
    }

    pub fn language_code(&self) -> Option<String> {
        self.inner.get_language_code().borrow().clone()
    }

    pub fn translate(&self, key: String) -> Option<String> {
        self.inner.translate(key)
    }
}

impl LocalizationClient {
    pub(crate) fn inner(&self) -> Arc<dyn Client> {
        self.inner.clone()
    }
}

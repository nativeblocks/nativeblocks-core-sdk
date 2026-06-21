use std::sync::Arc;

use crate::common::cache::new_cache_provider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::new_http_client;
use crate::common::result::NbError;
use crate::config;
use crate::localization::client::Client;
use crate::localization::data::source::new_local_source;
use crate::localization::di;
use crate::localization::key::operation;
use crate::localization::model::LocalizationSyncRequest;

/// UniFFI handle for the localization feature. Wraps the internal localization
/// `Client` and projects its surface across the FFI boundary. The real GraphQL
/// endpoint, the localization gateways and the install id are resolved from the
/// project config (`config::Client`).
#[derive(uniffi::Object)]
pub struct LocalizationClient {
    inner: Arc<Client>,
    config: Arc<config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl LocalizationClient {
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
        let local = new_local_source(&db_path)?;
        let inner = Arc::new(di::new_client(http, environment, config, local));
        return Ok(Arc::new(Self {
            inner,
            config: config_client,
        }));
    }

    pub async fn sync_localization(&self, language_code: String) -> Result<(), NbError> {
        let localization = self.config.gateway_for(operation::LOCALIZATIONS).await?;
        let production = self
            .config
            .gateway_for(operation::LOCALIZATIONS_PRODUCTION)
            .await?;
        let checksum = self
            .config
            .gateway_for(operation::PRODUCTION_CHECKSUM)
            .await?;
        let request = LocalizationSyncRequest {
            endpoint_localization: localization.gateway,
            endpoint_localization_production: production.gateway,
            endpoint_localization_production_checksum: checksum.gateway,
            graphql_endpoint: localization.endpoint,
            language_code,
            install_id: localization.install_id,
        };
        return self
            .inner
            .sync_localization(request)
            .await
            .map_err(NbError::from);
    }

    pub async fn load_localization(&self, language_code: String) -> Result<(), NbError> {
        return self
            .inner
            .get_localization(language_code)
            .await
            .map_err(NbError::from);
    }

    pub fn set_language_code(&self, language_code: String) {
        self.inner.set_language_code(language_code);
    }

    pub fn language_code(&self) -> Option<String> {
        return self.inner.get_language_code().borrow().clone();
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.inner.translate(key);
    }
}

impl LocalizationClient {
    pub(crate) fn inner(&self) -> Arc<Client> {
        return self.inner.clone();
    }
}

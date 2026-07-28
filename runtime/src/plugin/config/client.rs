use std::sync::{Arc, Mutex};

use tokio::sync::Mutex as AsyncMutex;

use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;
use crate::plugin::config::model::{NativeProjectConfigModel, ResolvedGatewayModel};
use crate::plugin::config::repository;

pub(crate) struct Client {
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    install_id: Mutex<Option<String>>,
    config: AsyncMutex<Option<NativeProjectConfigModel>>,
}

impl Client {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
    ) -> Self {
        return Self {
            http,
            cache,
            environment,
            sdk_config,
            install_id: Mutex::new(None),
            config: AsyncMutex::new(None),
        };
    }

    pub(crate) async fn gateway(&self, operation: &str) -> NBResult<ResolvedGatewayModel> {
        let install_id = self.install_id().await?;
        let config = self.project_config(&install_id).await?;
        let gateway = repository::resolve_gateway(&config, operation);
        return Ok(ResolvedGatewayModel {
            gateway,
            endpoint: config.endpoint,
            install_id,
        });
    }

    async fn install_id(&self) -> NBResult<String> {
        if let Some(id) = self.install_id.lock().unwrap().clone() {
            return Ok(id);
        }
        let id = repository::install_id(self.cache.as_ref()).await?;
        *self.install_id.lock().unwrap() = Some(id.clone());
        return Ok(id);
    }

    async fn project_config(&self, install_id: &str) -> NBResult<NativeProjectConfigModel> {
        let mut guard = self.config.lock().await;

        // In-memory copy is authoritative for this process.
        if let Some(config) = guard.as_ref() {
            return Ok(config.clone());
        }

        // Durable last-known-good config; never TTL-expired.
        let cached = repository::read_cached_config(self.cache.as_ref()).await?;

        // Still within the refresh window: serve cache, skip the network.
        if let Some(config) = &cached {
            if repository::is_config_fresh(self.cache.as_ref()).await? {
                *guard = Some(config.clone());
                return Ok(config.clone());
            }
        }

        // Stale or first run: try to refresh, but never lose a usable config.
        match repository::fetch_project_config(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            install_id,
        )
        .await
        {
            Ok(config) => {
                repository::write_cached_config(self.cache.as_ref(), &config).await?;
                *guard = Some(config.clone());
                Ok(config)
            }
            // Backend/offline/expired-key failure: fall back to the cached config.
            Err(error) => match cached {
                Some(config) => {
                    *guard = Some(config.clone());
                    Ok(config)
                }
                None => Err(error),
            },
        }
    }
}

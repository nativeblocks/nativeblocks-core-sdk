use std::sync::{Arc, Mutex};

use tokio::sync::Mutex as AsyncMutex;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config::model::{NativeProjectConfigModel, ResolvedGatewayModel};
use crate::config::repository;

pub(crate) struct Client {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    install_id: Mutex<Option<String>>,
    config: AsyncMutex<Option<NativeProjectConfigModel>>,
}

impl Client {
    pub(super) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            cache,
            install_id: Mutex::new(None),
            config: AsyncMutex::new(None),
        };
    }

    pub(crate) async fn gateway(&self, operation: &str) -> NBResult<ResolvedGatewayModel> {
        let install_id = self.install_id()?;
        let config = self.project_config(&install_id).await?;
        let gateway = repository::resolve_gateway(&config, operation);
        return Ok(ResolvedGatewayModel {gateway, endpoint: config.endpoint, install_id});
    }

    fn install_id(&self) -> NBResult<String> {
        if let Some(id) = self.install_id.lock().unwrap().clone() {
            return Ok(id);
        }
        let id = repository::install_id(self.cache.as_ref())?;
        *self.install_id.lock().unwrap() = Some(id.clone());
        return Ok(id);
    }

    async fn project_config(&self, install_id: &str) -> NBResult<NativeProjectConfigModel> {
        let mut guard = self.config.lock().await;
        if let Some(config) = guard.as_ref() {
            return Ok(config.clone());
        }
        if let Some(config) = repository::read_cached_config(self.cache.as_ref())? {
            *guard = Some(config.clone());
            return Ok(config);
        }
        let config = repository::fetch_project_config(self.http.as_ref(), &self.environment, &self.sdk_config, install_id).await?;
        repository::write_cached_config(self.cache.as_ref(), &config)?;
        *guard = Some(config.clone());
        return Ok(config);
    }
}

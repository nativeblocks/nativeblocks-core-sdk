use std::sync::Arc;

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
    mutex: AsyncMutex<Option<NativeProjectConfigModel>>,
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
            mutex: AsyncMutex::new(None),
        };
    }

    pub(crate) async fn gateway(&self, operation: &str) -> NBResult<ResolvedGatewayModel> {
        let install_id = repository::install_id(self.cache.as_ref())?;
        let config = self.project_config(&install_id).await?;
        let gateway = repository::resolve_gateway(&config, operation);
        return Ok(ResolvedGatewayModel {
            gateway,
            endpoint: config.endpoint,
            install_id,
        });
    }

    async fn project_config(&self, install_id: &str) -> NBResult<NativeProjectConfigModel> {
        let mut guard = self.mutex.lock().await;
        if let Some(config) = guard.as_ref() {
            return Ok(config.clone());
        }
        let config = repository::fetch_project_config(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            install_id
        ).await?;
        *guard = Some(config.clone());
        return Ok(config);
    }
}

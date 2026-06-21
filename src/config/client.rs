use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use tokio::sync::Mutex as AsyncMutex;

use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::net::{HttpClient, INSTALL_ID_HEADER, decode_envelope, with_headers};
use crate::common::result::{ErrorModel, NbResult};
use crate::config::data::dto::ProjectConfigDataDto;
use crate::config::key::{GRAPHQL_GATEWAY_TYPE, error_code, message};
use crate::config::model::NativeProjectConfigModel;

#[async_trait]
pub trait Client: Send + Sync {
    async fn project_config(&self, install_id: &str) -> NbResult<NativeProjectConfigModel>;
}

pub fn gateway_for(config: &NativeProjectConfigModel, operation: &str) -> ProjectConfigGateway {
    config
        .endpoints
        .iter()
        .find(|gateway| gateway.operation == operation)
        .cloned()
        .unwrap_or_else(|| ProjectConfigGateway {
            operation: operation.to_string(),
            gateway_type: GRAPHQL_GATEWAY_TYPE.to_string(),
            value: operation.to_string(),
        })
}

struct ClientImpl {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    cache: AsyncMutex<Option<NativeProjectConfigModel>>,
}

pub fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
) -> Arc<dyn Client> {
    Arc::new(ClientImpl {
        http,
        environment,
        config,
        cache: AsyncMutex::new(None),
    })
}

#[async_trait]
impl Client for ClientImpl {
    async fn project_config(&self, install_id: &str) -> NbResult<NativeProjectConfigModel> {
        let mut cache = self.cache.lock().await;
        if let Some(config) = cache.as_ref() {
            return Ok(config.clone());
        }
        let model = self.fetch(install_id).await?;
        *cache = Some(model.clone());
        Ok(model)
    }
}

impl ClientImpl {
    async fn fetch(&self, install_id: &str) -> NbResult<NativeProjectConfigModel> {
        let endpoint = self.environment.endpoint().ok_or_else(|| {
            ErrorModel::support(message::CLOUD_ONLY).with_code(error_code::PROJECT_CONFIG)
        })?;
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        let body = self.http.get(endpoint, &headers).await?;
        let dto: ProjectConfigDataDto = decode_envelope(&body)?;
        Ok(dto.to_model())
    }
}

fn registry() -> &'static Mutex<HashMap<String, Arc<dyn Client>>> {
    static REGISTRY: OnceLock<Mutex<HashMap<String, Arc<dyn Client>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn get_or_create(
    http: Arc<dyn HttpClient>,
    environment: &NativeblocksEnvironment,
    config: &SdkConfig,
) -> Arc<dyn Client> {
    let mut map = registry().lock().expect("config registry poisoned");
    map.entry(environment.instance_name().to_string())
        .or_insert_with(|| new_client(http, environment.clone(), config.clone()))
        .clone()
}

#[cfg(test)]
#[path = "client.test.rs"]
mod tests;

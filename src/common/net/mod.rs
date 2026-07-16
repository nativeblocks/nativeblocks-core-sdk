use std::collections::HashMap;

use serde::de::DeserializeOwned;

use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::result::{NBError, NBResult};

mod graphql;
mod mapper;
mod rest;

pub(crate) use graphql::{GraphQlRequest, GraphQlTransport, GATEWAY_TYPE_GRAPHQL};
pub(crate) use mapper::map;
pub(crate) use rest::{RestTransport, GATEWAY_TYPE_REST};

pub(crate) const API_KEY_HEADER: &str = "Api-Key";
pub(crate) const INSTALL_ID_HEADER: &str = "Install-Id";
pub(crate) const SDK_PLATFORM_HEADER: &str = "SDK-Platform";
pub(crate) const SDK_VERSION_HEADER: &str = "SDK-Version";

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(&self, url: String, headers: HashMap<String, String>) -> Result<String, NBError>;
    async fn post(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<String, NBError>;
}

pub(crate) fn with_headers(
    environment: &NativeblocksEnvironment,
    config: &SdkConfig,
    install_id: &str,
) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert(
        API_KEY_HEADER.to_string(),
        format!("Bearer {}", environment.api_key()),
    );
    headers.insert(SDK_VERSION_HEADER.to_string(), config.version.clone());
    headers.insert(SDK_PLATFORM_HEADER.to_string(), config.platform.clone());
    headers.insert(INSTALL_ID_HEADER.to_string(), install_id.to_string());
    return headers;
}

#[async_trait::async_trait]
pub(crate) trait GatewayTransport: Send + Sync {
    async fn send(
        &self,
        client: &dyn HttpClient,
        headers: HashMap<String, String>,
    ) -> NBResult<String>;
}

pub(crate) async fn request<D: DeserializeOwned>(
    client: &dyn HttpClient,
    headers: HashMap<String, String>,
    transport: &dyn GatewayTransport,
) -> NBResult<D> {
    let body = transport.send(client, headers).await?;
    return map::<D>(&body);
}

#[cfg(feature = "net-reqwest")]
mod reqwest_client;

use std::sync::Arc;

use async_trait::async_trait;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::dto::{BaseDto, BaseErrorDto};
use crate::common::result::{ErrorModel, NBResult};

pub(crate) const DEFAULT_TIMEOUT_SECS: u64 = 10;

pub(crate) const INSTALL_ID_HEADER: &str = "Install-Id";
pub(crate) const GATEWAY_TYPE_REST: &str = "rest";

pub(crate) type Header = (String, String);

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn post(&self, endpoint: &str, headers: &[Header], body: &str) -> NBResult<String>;
    async fn get(&self, endpoint: &str, headers: &[Header]) -> NBResult<String>;
}

#[cfg(feature = "net-reqwest")]
pub(crate) fn new_http_client() -> NBResult<Arc<dyn HttpClient>> {
    Ok(Arc::new(reqwest_client::ReqwestHttpClient::new()?))
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GraphQlRequest {
    pub query: String,
    pub variables: Value,
    pub operation_name: String,
}

impl GraphQlRequest {
    pub(crate) fn new(query: impl Into<String>) -> Self {
        let query = query.into();
        let operation_name = operation_name_from_query(&query);
        Self {
            query,
            variables: Value::Object(Default::default()),
            operation_name,
        }
    }

    pub(crate) fn with_variables(mut self, variables: Value) -> Self {
        self.variables = variables;
        self
    }
}

fn operation_name_from_query(query: &str) -> String {
    let mut tokens = query.split_whitespace();
    while let Some(token) = tokens.next() {
        if matches!(token, "query" | "mutation" | "subscription") {
            if let Some(next) = tokens.next() {
                let name: String = next.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                if !name.is_empty() {
                    return name;
                }
            }
        }
    }
    String::new()
}

pub(crate) fn with_headers(environment: &NativeblocksEnvironment, config: &SdkConfig) -> Vec<Header> {
    vec![
        ("Api-Key".to_string(),format!("Bearer {}", environment.api_key())),
        ("SDK-Version".to_string(), config.version.clone()),
        ("SDK-Platform".to_string(), config.platform.clone()),
    ]
}

pub(crate) async fn execute_graphql<D: DeserializeOwned>(
    client: &dyn HttpClient,
    endpoint: &str,
    headers: &[Header],
    request: &GraphQlRequest,
) -> NBResult<D> {
    let body = serde_json::to_string(request)
.map_err(|e| ErrorModel::network(format!("Failed to encode request: {e}")))?;
    let response = client.post(endpoint, headers, &body).await?;
    decode_envelope::<D>(&response)
}

pub(crate) fn decode_envelope<D: DeserializeOwned>(response: &str) -> NBResult<D> {
    let dto: BaseDto<D> = serde_json::from_str(response)
        .map_err(|e| ErrorModel::network(format!("Failed to decode response: {e}")))?;

    if let Some(errors) = dto.errors.as_ref().filter(|e| !e.is_empty()) {
        return Err(map_graphql_errors(errors));
    }
    dto.data
        .ok_or_else(|| ErrorModel::network("Please try again"))
}

fn map_graphql_errors(errors: &[BaseErrorDto]) -> ErrorModel {
    match errors.first() {
        Some(error) => ErrorModel::network(error.message.clone())
            .with_code(error.extensions.classification.clone()),
        None => ErrorModel::network("Please try again"),
    }
}

use std::collections::HashMap;

use crate::library::net::network::{GatewayTransport, HttpClient};
use crate::library::result::{ErrorModel, NBResult};
use serde::Serialize;
use serde_json::Value;

pub(crate) const GATEWAY_TYPE_GRAPHQL: &str = "graphql";

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
        return Self {
            query,
            variables: Value::Object(Default::default()),
            operation_name,
        };
    }

    pub(crate) fn with_variables(mut self, variables: Value) -> Self {
        self.variables = variables;
        return self;
    }
}

fn operation_name_from_query(query: &str) -> String {
    let mut tokens = query.split_whitespace();
    while let Some(token) = tokens.next() {
        if matches!(token, "query" | "mutation" | "subscription") {
            if let Some(next) = tokens.next() {
                let name: String = next
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_')
                    .collect();
                if !name.is_empty() {
                    return name;
                }
            }
        }
    }
    return String::new();
}

pub(crate) struct GraphQlTransport {
    endpoint: String,
    request: GraphQlRequest,
}

impl GraphQlTransport {
    pub(crate) fn new(endpoint: impl Into<String>, request: GraphQlRequest) -> Self {
        return Self {
            endpoint: endpoint.into(),
            request,
        };
    }
}

#[async_trait::async_trait]
impl GatewayTransport for GraphQlTransport {
    async fn send(
        &self,
        client: &dyn HttpClient,
        headers: HashMap<String, String>,
    ) -> NBResult<String> {
        let body = serde_json::to_string(&self.request)
            .map_err(|e| ErrorModel::network(format!("Failed to encode request: {e}")))?;
        return Ok(client.post(self.endpoint.clone(), headers, body).await?);
    }
}

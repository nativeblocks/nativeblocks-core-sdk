use std::sync::Arc;

use serde_json::json;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, HttpClient, INSTALL_ID_HEADER, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::NBResult;
use crate::experiment::data::dto::NativeExperimentDataDto;
use crate::experiment::graphql;
use crate::experiment::model::ExperimentRequest;

pub(crate) struct ExperimentRemoteSource {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
}

impl ExperimentRemoteSource {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Self {
        return Self {
            http,
            environment,
            config,
        };
    }

    pub(crate) async fn fetch(
        &self,
        request: &ExperimentRequest,
    ) -> NBResult<NativeExperimentDataDto> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), request.install_id.clone()));

        if request.gateway.gateway_type == GATEWAY_TYPE_REST {
            let body = self.http.get(&rest_url(request), &headers).await?;
            return decode_envelope::<NativeExperimentDataDto>(&body);
        }

        let gql = GraphQlRequest::new(graphql::EXPERIMENT_QUERY)
            .with_variables(graphql_variables(request));
        return execute_graphql::<NativeExperimentDataDto>(
            self.http.as_ref(),
            &request.graphql_endpoint,
            &headers,
            &gql,
        )
        .await;
    }
}

fn graphql_variables(request: &ExperimentRequest) -> serde_json::Value {
    let variables: Vec<_> = request
        .parameters
        .iter()
        .map(|(k, v)| json!({ "key": k, "value": v }))
        .collect();
    return json!({
        "key": request.key,
        "parameter": { "variables": variables },
    });
}

fn rest_url(request: &ExperimentRequest) -> String {
    let mut url = format!("{}?key={}", request.gateway.value, encode_query(&request.key));
    if !request.parameters.is_empty() {
        let parameters_json = serde_json::to_string(&request.parameters).unwrap_or_default();
        url.push_str(&format!("&parameters={}", encode_query(&parameters_json)));
    }
    return url;
}

fn encode_query(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    return out;
}

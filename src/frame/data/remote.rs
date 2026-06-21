use std::collections::HashMap;
use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, Header, HttpClient, INSTALL_ID_HEADER, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::NBResult;
use crate::frame::data::dto::{NativeFrameDataDto, NativeFrameProductionChecksumDataDto};
use crate::frame::data::mapper::frame_to_model;
use crate::frame::data::graphql;
use crate::frame::domain::model::NativeFrameModel;

pub(crate) struct FrameRemoteSource {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
}

impl FrameRemoteSource {
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

    fn headers(&self, install_id: &str) -> Vec<Header> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        return headers;
    }

    pub(crate) async fn fetch_frame(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        route: &str,
        install_id: &str,
        production: bool,
        parameters: &HashMap<String, String>,
    ) -> NBResult<NativeFrameModel> {
        let headers = self.headers(install_id);

        let dto: NativeFrameDataDto = if gateway.gateway_type == GATEWAY_TYPE_REST {
            let url = rest_url(&gateway.value, route, parameters);
            let body = self.http.get(&url, &headers).await?;
            decode_envelope::<NativeFrameDataDto>(&body)?
        } else {
            let query = if production {
                graphql::FRAME_PRODUCTION_QUERY
            } else {
                graphql::FRAME_QUERY
            };
            let request =
                GraphQlRequest::new(query).with_variables(graphql::frame_variables(route, parameters));
            execute_graphql::<NativeFrameDataDto>(
                self.http.as_ref(),
                graphql_endpoint,
                &headers,
                &request,
            )
            .await?
        };

        let model = if production {
            frame_to_model(dto.frame_production.as_ref())
        } else {
            frame_to_model(dto.frame.as_ref())
        };
        return Ok(model);
    }

    pub(crate) async fn fetch_checksum(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        route: &str,
        install_id: &str,
        parameters: &HashMap<String, String>,
    ) -> NBResult<String> {
        let headers = self.headers(install_id);

        let dto: NativeFrameProductionChecksumDataDto =
            if gateway.gateway_type == GATEWAY_TYPE_REST {
                let url = rest_url(&gateway.value, route, parameters);
                let body = self.http.get(&url, &headers).await?;
                decode_envelope::<NativeFrameProductionChecksumDataDto>(&body)?
            } else {
                let request = GraphQlRequest::new(graphql::FRAME_PRODUCTION_CHECKSUM_QUERY)
                    .with_variables(graphql::frame_variables(route, parameters));
                execute_graphql::<NativeFrameProductionChecksumDataDto>(
                    self.http.as_ref(),
                    graphql_endpoint,
                    &headers,
                    &request,
                )
                .await?
            };

        return Ok(dto
            .frame_production_checksum
            .and_then(|c| c.checksum)
            .unwrap_or_default());
    }

    pub(crate) async fn fetch_community(
        &self,
        endpoint_frame: &str,
    ) -> NBResult<NativeFrameModel> {
        let body = self.http.get(endpoint_frame, &[]).await?;
        let dto = decode_envelope::<NativeFrameDataDto>(&body)?;
        return Ok(frame_to_model(dto.frame_production.as_ref()));
    }
}

fn rest_url(base: &str, route: &str, parameters: &HashMap<String, String>) -> String {
    let separator = if base.contains('?') { '&' } else { '?' };
    let mut url = format!("{base}{separator}route={}", encode(route));
    if !parameters.is_empty() {
        let json = serde_json::to_string(parameters).unwrap_or_default();
        url.push_str(&format!("&parameters={}", encode(&json)));
    }
    return url;
}

fn encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    return encoded;
}

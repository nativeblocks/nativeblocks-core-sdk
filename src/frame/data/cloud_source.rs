use std::collections::HashMap;

use serde_json::{Value, json};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{self, GatewayTransport, GraphQlRequest, HttpClient, with_headers};
use crate::common::result::NBResult;
use crate::config::ProjectConfigGatewayModel;
use crate::frame::data::db_source;
use crate::frame::data::dto::{NativeFrameDataDto, NativeFrameProductionChecksumDataDto};
use crate::frame::data::key::{self, error_code};
use crate::frame::data::mapper;
use crate::frame::data::query;
use crate::frame::domain::model::NativeFrameModel;

pub(super) async fn sync_cloud(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    frame_gateway: ProjectConfigGatewayModel,
    frame_production_gateway: ProjectConfigGatewayModel,
    frame_production_checksum_gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<NativeFrameModel> {
    if environment.development_mode() {
        return fetch_dev_frame(
            http,
            environment,
            sdk_config,
            cache,
            frame_gateway,
            graphql_endpoint,
            install_id,
            route,
            parameters,
        )
        .await;
    }

    return match db_source::cached_checksum(cache, route)? {
        None => {
            fetch_production_frame(
                http,
                environment,
                sdk_config,
                cache,
                frame_production_gateway,
                graphql_endpoint,
                install_id,
                route,
                parameters,
            )
            .await
        }
        Some(cached) => {
            let remote = fetch_production_checksum(
                http,
                environment,
                sdk_config,
                frame_production_checksum_gateway,
                graphql_endpoint,
                install_id,
                route,
                parameters,
            )
            .await?;
            if remote != cached {
                fetch_production_frame(
                    http,
                    environment,
                    sdk_config,
                    cache,
                    frame_production_gateway,
                    graphql_endpoint,
                    install_id,
                    route,
                    parameters,
                )
                .await
            } else {
                db_source::get_frame(cache, route, false)
            }
        }
    };
}

async fn fetch_dev_frame(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<NativeFrameModel> {
    let frame = request_frame(
        http,
        environment,
        sdk_config,
        &gateway,
        graphql_endpoint,
        install_id,
        route,
        parameters,
        query::FRAME_QUERY,
        false,
    )
    .await
    .map_err(|error| error.or_code(error_code::FRAME_DEV_SYNC))?;
    db_source::save_frame(cache, route, &frame, false)?;
    return Ok(frame);
}

async fn fetch_production_frame(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<NativeFrameModel> {
    let frame = request_frame(
        http,
        environment,
        sdk_config,
        &gateway,
        graphql_endpoint,
        install_id,
        route,
        parameters,
        query::FRAME_PRODUCTION_QUERY,
        true,
    )
    .await
    .map_err(|error| error.or_code(error_code::FRAME_PRODUCTION_SYNC))?;
    db_source::save_frame(cache, route, &frame, true)?;
    return Ok(frame);
}

async fn fetch_production_checksum(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<String> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(
        &gateway,
        graphql_endpoint,
        route,
        parameters,
        query::FRAME_PRODUCTION_CHECKSUM_QUERY,
    );
    let data: NativeFrameProductionChecksumDataDto = net::request(http, headers, transport.as_ref()).await
            .map_err(|error| error.or_code(error_code::FRAME_CHECKSUM))?;
    return Ok(data
        .frame_production_checksum
        .and_then(|checksum| checksum.checksum)
        .unwrap_or_default());
}

async fn request_frame(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    query: &str,
    production: bool,
) -> NBResult<NativeFrameModel> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(gateway, graphql_endpoint, route, parameters, query);
    let data: NativeFrameDataDto = net::request(http, headers, transport.as_ref()).await?;
    let frame = if production { data.frame_production } else { data.frame };
    return Ok(mapper::to_model(frame.as_ref()));
}

fn build_transport(
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    query: &str,
) -> Box<dyn GatewayTransport> {
    return match gateway.gateway_type.as_str() {
        net::GATEWAY_TYPE_REST => {
            let mut variables = vec![(key::PARAM_ROUTE.to_string(), route.to_string())];
            if !parameters.is_empty() {
                variables.push((key::PARAM_PARAMETERS.to_string(), encode_parameters(parameters)));
            }
            Box::new(net::RestTransport::new(gateway.value.clone(), variables))
        }
        net::GATEWAY_TYPE_GRAPHQL | _ => {
            let request = GraphQlRequest::new(query).with_variables(graphql_variables(route, parameters));
            Box::new(net::GraphQlTransport::new(graphql_endpoint, request))
        }
    };
}

fn graphql_variables(route: &str, parameters: &HashMap<String, String>) -> Value {
    let params: Vec<Value> = parameters
        .iter()
        .map(|(key, value)| json!({ "key": key, "value": value }))
        .collect();
    return json!({
        "route": route,
        "parameter": { "variables": params }
    });
}

fn encode_parameters(parameters: &HashMap<String, String>) -> String {
    return serde_json::to_string(parameters).unwrap_or_else(|_| "{}".to_string());
}


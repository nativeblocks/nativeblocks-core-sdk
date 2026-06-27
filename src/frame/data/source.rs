use std::collections::HashMap;

use serde_json::{Value, json};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::json;
use crate::common::net::{self, GatewayTransport, GraphQlRequest, HttpClient, with_headers};
use crate::common::result::{ErrorModel, NBResult};
use crate::config::ProjectConfigGatewayModel;
use crate::frame::data::dto::{NativeFrameDataDto, NativeFrameProductionChecksumDataDto};
use crate::frame::data::key::{self, error_code};
use crate::frame::data::mapper;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::data::query;

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

    return match cached_checksum(cache, route)? {
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
                get_frame(cache, route, false)
            }
        }
    };
}

pub(super) fn get_frame(
    cache: &dyn CacheProvider,
    route: &str,
    development_mode: bool,
) -> NBResult<NativeFrameModel> {
    let cache_key = if development_mode {
        key::dev_key(route)
    } else {
        key::prod_key(route)
    };
    return match cache.get_bytes(cache_key)? {
        Some(bytes) => decode_frame(&bytes),
        None => Err(ErrorModel::cache(key::message::FRAME_NOT_CACHED).with_code(error_code::FRAME_NOT_CACHED)),
    };
}

pub(super) async fn clear(cache: &dyn CacheProvider, route: &str) -> NBResult<()> {
    cache.remove(key::dev_key(route))?;
    cache.remove(key::prod_key(route))?;
    return Ok(());
}

pub(super) async fn clear_all(cache: &dyn CacheProvider, routes: &[String]) -> NBResult<()> {
    for route in routes {
        clear(cache, route).await?;
    }
    return Ok(());
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
    cache_frame(cache, route, &frame, false)?;
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
    cache_frame(cache, route, &frame, true)?;
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

fn cached_checksum(cache: &dyn CacheProvider, route: &str) -> NBResult<Option<String>> {
    return match cache.get_bytes(key::prod_key(route))? {
        Some(bytes) => Ok(decode_frame(&bytes)?.checksum),
        None => Ok(None),
    };
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

fn cache_frame(
    cache: &dyn CacheProvider,
    route: &str,
    frame: &NativeFrameModel,
    production: bool,
) -> NBResult<()> {
    let cache_key = if production { key::prod_key(route) } else { key::dev_key(route) };
    let bytes = json::to_bytes(frame)?;
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

fn decode_frame(bytes: &[u8]) -> NBResult<NativeFrameModel> {
    return json::from_bytes(bytes).map_err(|error| error.with_code(error_code::FRAME_NOT_CACHED));
}

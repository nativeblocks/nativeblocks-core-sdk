use std::collections::HashMap;

use crate::feature::frame::data::db::db_source;
use crate::feature::frame::data::key::{self, error_code};
use crate::feature::frame::data::mapper;
use crate::feature::frame::data::network::dto::{
    NativeFrameDataDto, NativeFrameProductionChecksumDataDto,
};
use crate::feature::frame::data::network::query;
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::{GatewayTransport, HttpClient, request, with_headers};
use crate::library::net::{
    GATEWAY_TYPE_GRAPHQL, GATEWAY_TYPE_REST, GraphQlRequest, GraphQlTransport, RestTransport,
};
use crate::library::result::NBResult;
use crate::plugin::config::ProjectConfigGatewayModel;
use serde_json::{Value, json};

pub(in crate::feature::frame::data) enum SyncOutcome {
    Updated(NativeFrameModel),
    Unchanged,
}

pub(in crate::feature::frame::data) async fn sync_cloud(
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
) -> NBResult<SyncOutcome> {
    if environment.development_mode() {
        let frame = fetch_dev_frame(
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
        .await?;
        return Ok(SyncOutcome::Updated(frame));
    }

    let outcome = match db_source::cached_checksum(cache, route).await? {
        None => SyncOutcome::Updated(
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
            .await?,
        ),
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
                SyncOutcome::Updated(
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
                    .await?,
                )
            } else {
                SyncOutcome::Unchanged
            }
        }
    };
    return Ok(outcome);
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
    db_source::save_frame(cache, route, &frame, false).await?;
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
    db_source::save_frame(cache, route, &frame, true).await?;
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
    let data: NativeFrameProductionChecksumDataDto = request(http, headers, transport.as_ref())
        .await
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
    let data: NativeFrameDataDto = request(http, headers, transport.as_ref()).await?;
    let frame = if production {
        data.frame_production
    } else {
        data.frame
    };
    return Ok(mapper::to_model(frame));
}

fn build_transport(
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    query: &str,
) -> Box<dyn GatewayTransport> {
    return match gateway.gateway_type.as_str() {
        GATEWAY_TYPE_REST => {
            let mut variables = vec![(key::PARAM_ROUTE.to_string(), route.to_string())];
            if !parameters.is_empty() {
                variables.push((
                    key::PARAM_PARAMETERS.to_string(),
                    encode_parameters(parameters),
                ));
            }
            Box::new(RestTransport::new(gateway.value.clone(), variables))
        }
        GATEWAY_TYPE_GRAPHQL | _ => {
            let request =
                GraphQlRequest::new(query).with_variables(graphql_variables(route, parameters));
            Box::new(GraphQlTransport::new(graphql_endpoint, request))
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

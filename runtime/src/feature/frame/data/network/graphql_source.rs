use std::collections::HashMap;

use serde_json::{Value, json};

use crate::feature::frame::data::mapper;
use crate::feature::frame::data::network::dto::{
    NativeFrameDataDto, NativeFrameProductionChecksumDataDto,
};
use crate::feature::frame::data::network::query;
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::library::net::{GraphQlRequest, GraphQlTransport, HttpClient, request};
use crate::library::result::NBResult;

pub(in crate::feature::frame::data) async fn frame(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<NativeFrameModel> {
    let data = request_frame(
        http,
        headers,
        endpoint,
        route,
        parameters,
        query::FRAME_QUERY,
    )
    .await?;
    return Ok(mapper::to_model(data.frame));
}

pub(in crate::feature::frame::data) async fn frame_production(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<NativeFrameModel> {
    let data = request_frame(
        http,
        headers,
        endpoint,
        route,
        parameters,
        query::FRAME_PRODUCTION_QUERY,
    )
    .await?;
    return Ok(mapper::to_model(data.frame_production));
}

pub(in crate::feature::frame::data) async fn frame_production_checksum(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
) -> NBResult<String> {
    let transport = transport(
        endpoint,
        route,
        parameters,
        query::FRAME_PRODUCTION_CHECKSUM_QUERY,
    );
    let data: NativeFrameProductionChecksumDataDto = request(http, headers, &transport).await?;
    return Ok(data
        .frame_production_checksum
        .and_then(|checksum| checksum.checksum)
        .unwrap_or_default());
}

async fn request_frame(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    query: &str,
) -> NBResult<NativeFrameDataDto> {
    let transport = transport(endpoint, route, parameters, query);
    return request(http, headers, &transport).await;
}

fn transport(
    endpoint: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    query: &str,
) -> GraphQlTransport {
    let params: Vec<Value> = parameters
        .iter()
        .map(|(key, value)| json!({ "key": key, "value": value }))
        .collect();
    let variables = json!({
        "route": route,
        "parameter": { "variables": params }
    });
    return GraphQlTransport::new(
        endpoint,
        GraphQlRequest::new(query).with_variables(variables),
    );
}

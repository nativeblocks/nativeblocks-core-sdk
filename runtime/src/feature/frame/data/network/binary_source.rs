use std::collections::HashMap;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;

use crate::feature::frame::data::key;
use crate::feature::frame::data::mapper;
use crate::feature::frame::data::network::dto::NativeFrameDto;
use crate::feature::frame::domain::model::NativeFrameModel;
use crate::library::net::{BinaryError, BinaryRequest, BinaryResponse, HttpClient, fetch_binary};
use crate::library::result::ErrorModel;

pub(in crate::feature::frame::data) async fn frame(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    url: &str,
    route: &str,
) -> Result<NativeFrameModel, BinaryError> {
    let request = BinaryRequest {
        url,
        query: vec![(key::PARAM_ROUTE, route.to_string())],
        headers,
        etag: None,
    };
    return match fetch_binary(http, request, &key::BINARY_FORMAT, route).await? {
        BinaryResponse::Modified(payload) => to_model(&payload),
        BinaryResponse::NotModified => Err(BinaryError::Fallback(ErrorModel::network(
            "unexpected 304 for a development frame",
        ))),
    };
}

pub(in crate::feature::frame::data) async fn frame_production(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    url: &str,
    route: &str,
    parameters: &HashMap<String, String>,
    cached_checksum: Option<&str>,
) -> Result<Option<NativeFrameModel>, BinaryError> {
    let mut query = vec![(key::PARAM_ROUTE, route.to_string())];
    if !parameters.is_empty() {
        let variables = serde_json::to_vec(parameters).unwrap_or_default();
        query.push((key::PARAM_PARAMETERS, URL_SAFE_NO_PAD.encode(&variables)));
    }
    let request = BinaryRequest {
        url,
        query,
        headers,
        etag: cached_checksum,
    };
    return match fetch_binary(http, request, &key::BINARY_FORMAT, route).await? {
        BinaryResponse::Modified(payload) => to_model(&payload).map(Some),
        BinaryResponse::NotModified => Ok(None),
    };
}

fn to_model(payload: &[u8]) -> Result<NativeFrameModel, BinaryError> {
    let dto: NativeFrameDto = serde_json::from_slice(payload).map_err(|error| {
        BinaryError::Fallback(ErrorModel::network(format!(
            "invalid frame payload: {error}"
        )))
    })?;
    return Ok(mapper::to_model(Some(dto)));
}

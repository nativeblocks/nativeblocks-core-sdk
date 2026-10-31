use std::collections::HashMap;

use serde::Deserialize;

use crate::library::net::dto::BaseErrorDto;
use crate::library::net::network::{HttpBytesResponse, HttpClient};
use crate::library::net::rest::percent_encode;
use crate::library::result::ErrorModel;

pub(crate) const GATEWAY_TYPE_BINARY: &str = "binary";

const IF_NONE_MATCH_HEADER: &str = "If-None-Match";
const PREFIX_SIZE: usize = 12;
const COMPRESSION_NONE: u8 = 0;

pub(crate) struct BinaryFormat {
    pub(crate) magic: [u8; 4],
    pub(crate) version: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BinaryResponse {
    Modified(Vec<u8>),
    NotModified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BinaryError {
    Fallback(ErrorModel),
    Failed(ErrorModel),
}

pub(crate) struct BinaryRequest<'a> {
    pub(crate) url: &'a str,
    pub(crate) query: Vec<(&'a str, String)>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) etag: Option<&'a str>,
}

pub(crate) async fn fetch(
    client: &dyn HttpClient,
    request: BinaryRequest<'_>,
    format: &BinaryFormat,
    key: &str,
) -> Result<BinaryResponse, BinaryError> {
    let mut headers = request.headers;
    if let Some(etag) = request.etag.filter(|etag| !etag.is_empty()) {
        headers.insert(IF_NONE_MATCH_HEADER.to_string(), format!("\"{etag}\""));
    }
    let url = with_query(request.url, &request.query);
    let response = client
        .get_bytes(url, headers)
        .await
        .map_err(|error| BinaryError::Failed(error.into()))?;
    return read_response(response, format, key);
}

fn read_response(
    response: HttpBytesResponse,
    format: &BinaryFormat,
    key: &str,
) -> Result<BinaryResponse, BinaryError> {
    let HttpBytesResponse { status, body } = response;
    return match status {
        304 => Ok(BinaryResponse::NotModified),
        200 => decode(format, &body, key)
            .map(BinaryResponse::Modified)
            .map_err(|message| BinaryError::Fallback(ErrorModel::network(message))),
        status if status >= 500 => Err(BinaryError::Fallback(
            api_error(&body).unwrap_or_else(|| ErrorModel::network(format!("HTTP {status}"))),
        )),
        status => match api_error(&body) {
            Some(error) if (400..500).contains(&status) => Err(BinaryError::Failed(error)),
            _ => Err(BinaryError::Fallback(ErrorModel::network(format!(
                "HTTP {status}"
            )))),
        },
    };
}

#[derive(Deserialize)]
struct ErrorBodyDto {
    #[serde(default)]
    errors: Vec<BaseErrorDto>,
}

fn api_error(body: &[u8]) -> Option<ErrorModel> {
    let dto: ErrorBodyDto = serde_json::from_slice(body).ok()?;
    let error = dto.errors.into_iter().next()?;
    return Some(ErrorModel::network(error.message).with_code(error.extensions.classification));
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HeaderDto {
    key: Option<String>,
    uncompressed_size: Option<usize>,
}

pub(crate) fn decode(format: &BinaryFormat, data: &[u8], key: &str) -> Result<Vec<u8>, String> {
    if data.len() < PREFIX_SIZE || data[..4] != format.magic {
        return Err("not a binary file of the expected format".to_string());
    }
    if data[4] != format.version {
        return Err(format!("unsupported version {}", data[4]));
    }
    if data[5] != COMPRESSION_NONE {
        return Err(format!("unsupported compression {}", data[5]));
    }
    let header_length = u16::from_le_bytes([data[6], data[7]]) as usize;
    let payload_length = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
    let payload_start = PREFIX_SIZE + header_length;
    let Some(payload_end) = payload_start.checked_add(payload_length) else {
        return Err("truncated file".to_string());
    };
    if data.len() < payload_end {
        return Err("truncated file".to_string());
    }

    let header: HeaderDto = serde_json::from_slice(&data[PREFIX_SIZE..payload_start])
        .map_err(|error| format!("invalid header: {error}"))?;
    if header.key.as_deref() != Some(key) {
        return Err(format!(
            "file is for '{}', expected '{key}'",
            header.key.unwrap_or_default()
        ));
    }
    if header.uncompressed_size != Some(payload_length) {
        return Err(format!(
            "payload is {payload_length} bytes, header says {:?}",
            header.uncompressed_size
        ));
    }
    return Ok(data[payload_start..payload_end].to_vec());
}

fn with_query(url: &str, query: &[(&str, String)]) -> String {
    let mut url = url.to_string();
    for (name, value) in query {
        let separator = if url.contains('?') { '&' } else { '?' };
        url.push(separator);
        url.push_str(name);
        url.push('=');
        url.push_str(&percent_encode(value));
    }
    return url;
}

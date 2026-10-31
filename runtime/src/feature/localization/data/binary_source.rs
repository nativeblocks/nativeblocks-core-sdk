use std::collections::HashMap;

use crate::feature::localization::data::dto::NativeLocalizationsDto;
use crate::feature::localization::data::key;
use crate::feature::localization::data::mapper;
use crate::feature::localization::domain::model::NativeLocalizationModel;
use crate::library::net::{BinaryError, BinaryRequest, BinaryResponse, HttpClient, fetch_binary};
use crate::library::result::ErrorModel;

pub(super) async fn localization(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    url: &str,
    language_code: &str,
) -> Result<NativeLocalizationModel, BinaryError> {
    return match fetch(http, headers, url, language_code, None).await? {
        Some(localization) => Ok(localization),
        None => Err(BinaryError::Fallback(ErrorModel::network(
            "unexpected 304 for a development localization",
        ))),
    };
}

pub(super) async fn localization_production(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    url: &str,
    language_code: &str,
    cached_checksum: Option<&str>,
) -> Result<Option<NativeLocalizationModel>, BinaryError> {
    return fetch(http, headers, url, language_code, cached_checksum).await;
}

async fn fetch(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    url: &str,
    language_code: &str,
    etag: Option<&str>,
) -> Result<Option<NativeLocalizationModel>, BinaryError> {
    let request = BinaryRequest {
        url,
        query: vec![(key::PARAM_LANGUAGE_CODE, language_code.to_string())],
        headers,
        etag,
    };
    return match fetch_binary(http, request, &key::BINARY_FORMAT, language_code).await? {
        BinaryResponse::Modified(payload) => {
            let dto: NativeLocalizationsDto =
                serde_json::from_slice(&payload).map_err(|error| {
                    BinaryError::Fallback(ErrorModel::network(format!(
                        "invalid localization payload: {error}"
                    )))
                })?;
            Ok(Some(mapper::to_model(Some(&dto))))
        }
        BinaryResponse::NotModified => Ok(None),
    };
}

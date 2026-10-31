use std::collections::HashMap;

use serde_json::json;

use crate::feature::localization::data::dto::{
    NativeLocalizationDataDto, NativeLocalizationProductionChecksumDataDto,
};
use crate::feature::localization::data::mapper;
use crate::feature::localization::data::query;
use crate::feature::localization::domain::model::NativeLocalizationModel;
use crate::library::net::{GraphQlRequest, GraphQlTransport, HttpClient, request};
use crate::library::result::NBResult;

pub(super) async fn localization(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    let data: NativeLocalizationDataDto = request(
        http,
        headers,
        &transport(endpoint, language_code, query::LOCALIZATION_QUERY),
    )
    .await?;
    return Ok(mapper::to_model(data.localizations.as_ref()));
}

pub(super) async fn localization_production(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    let data: NativeLocalizationDataDto = request(
        http,
        headers,
        &transport(
            endpoint,
            language_code,
            query::LOCALIZATION_PRODUCTION_QUERY,
        ),
    )
    .await?;
    return Ok(mapper::to_model(data.localizations_production.as_ref()));
}

pub(super) async fn localization_production_checksum(
    http: &dyn HttpClient,
    headers: HashMap<String, String>,
    endpoint: &str,
    language_code: &str,
) -> NBResult<String> {
    let data: NativeLocalizationProductionChecksumDataDto = request(
        http,
        headers,
        &transport(
            endpoint,
            language_code,
            query::LOCALIZATION_PRODUCTION_CHECKSUM_QUERY,
        ),
    )
    .await?;
    return Ok(data
        .localization_production_checksum
        .and_then(|checksum| checksum.checksum)
        .unwrap_or_default());
}

fn transport(endpoint: &str, language_code: &str, query: &str) -> GraphQlTransport {
    let request =
        GraphQlRequest::new(query).with_variables(json!({ "languageCode": language_code }));
    return GraphQlTransport::new(endpoint, request);
}

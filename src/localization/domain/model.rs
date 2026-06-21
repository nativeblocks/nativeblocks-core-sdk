use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::config::ProjectConfigGateway;

#[derive(Debug, Clone)]
pub(crate) struct LocalizationSyncRequest {
    pub endpoint_localization: ProjectConfigGateway,
    pub endpoint_localization_production: ProjectConfigGateway,
    pub endpoint_localization_production_checksum: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub language_code: String,
    pub install_id: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct NativeLocalizationModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub localizations: Option<HashMap<String, String>>,
}

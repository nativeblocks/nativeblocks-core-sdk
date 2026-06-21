use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::config::ProjectConfigGateway;

#[derive(Debug, Clone)]
pub(crate) struct ExperimentRequest {
    pub gateway: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub install_id: String,
    pub key: String,
    pub parameters: HashMap<String, String>,
    pub cache_ttl_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeExperimentModel {
    pub value: String,
    pub variable_type: String,
}

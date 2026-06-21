use serde::Deserialize;

use crate::common::config::ProjectConfigGateway;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfigDataDto {
    pub project_config: Option<ProjectConfigDto>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectConfigDto {
    #[serde(default)]
    pub gateway: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub endpoints: Option<Vec<ProjectConfigGateway>>,
}

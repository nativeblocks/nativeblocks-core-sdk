use serde::Deserialize;

use crate::plugin::config::model::ProjectConfigGatewayModel;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProjectConfigDataDto {
    pub(super) project_config: Option<ProjectConfigDto>,
}

#[derive(Deserialize)]
pub(super) struct ProjectConfigDto {
    #[serde(default)]
    pub(super) gateway: Option<String>,
    #[serde(default)]
    pub(super) endpoint: Option<String>,
    #[serde(default)]
    pub(super) endpoints: Option<Vec<ProjectConfigGatewayModel>>,
}

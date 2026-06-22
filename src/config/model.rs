use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub(crate) struct ProjectConfigGatewayModel {
    pub operation: String,
    #[serde(rename = "type")]
    pub gateway_type: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NativeProjectConfigModel {
    pub gateway: String,
    pub endpoint: String,
    pub endpoints: Vec<ProjectConfigGatewayModel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedGatewayModel {
    pub gateway: ProjectConfigGatewayModel,
    pub endpoint: String,
    pub install_id: String,
}

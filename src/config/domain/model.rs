use crate::common::config::ProjectConfigGateway;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeProjectConfigModel {
    pub gateway: String,
    pub endpoint: String,
    pub endpoints: Vec<ProjectConfigGateway>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedGateway {
    pub gateway: ProjectConfigGateway,
    pub endpoint: String,
    pub install_id: String,
}

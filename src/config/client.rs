use crate::common::result::NBResult;
use crate::config::data::repository::ProjectConfigRepository;
use crate::config::domain::resolve_gateway::get_gateway_use_case;
use crate::config::model::ResolvedGateway;

pub(crate) struct Client {
    repository: ProjectConfigRepository,
}

impl Client {
    pub(crate) fn new(repository: ProjectConfigRepository) -> Self {
        return Self { repository };
    }

    pub(crate) async fn gateway_for(&self, operation: &str) -> NBResult<ResolvedGateway> {
        return get_gateway_use_case(&self.repository, operation).await;
    }
}

use crate::common::config::ProjectConfigGateway;
use crate::common::result::NBResult;
use crate::config::data::repository::ProjectConfigRepository;
use crate::config::key::GRAPHQL_GATEWAY_TYPE;
use crate::config::model::ResolvedGateway;

pub(crate) async fn get_gateway_use_case(
    repository: &ProjectConfigRepository,
    operation: &str,
) -> NBResult<ResolvedGateway> {
    let install_id = repository.install_id()?;
    let config = repository.project_config(&install_id).await?;
    let gateway = config
        .endpoints
        .iter()
        .find(|gateway| gateway.operation == operation)
        .cloned()
        .unwrap_or_else(|| ProjectConfigGateway {
            operation: operation.to_string(),
            gateway_type: GRAPHQL_GATEWAY_TYPE.to_string(),
            value: operation.to_string(),
        });
    return Ok(ResolvedGateway {
        gateway,
        endpoint: config.endpoint,
        install_id,
    });
}

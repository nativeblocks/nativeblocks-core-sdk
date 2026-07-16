use crate::feature::scaffold::data::dto::ScaffoldDataDto;
use crate::feature::scaffold::data::mapper::to_model;
use crate::feature::scaffold::data::query::SCAFFOLD_QUERY;
use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::{self, with_headers, GraphQlRequest, HttpClient};
use crate::library::result::NBResult;
use crate::plugin::config::ProjectConfigGatewayModel;

pub(super) async fn fetch_scaffold(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
) -> NBResult<ScaffoldModel> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(&gateway, graphql_endpoint);
    let dto: ScaffoldDataDto = net::request(http, headers, transport.as_ref()).await?;
    return Ok(to_model(&dto));
}

fn build_transport(
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
) -> Box<dyn net::GatewayTransport> {
    return match gateway.gateway_type.as_str() {
        net::GATEWAY_TYPE_REST => {
            Box::new(net::RestTransport::new(gateway.value.clone(), Vec::new()))
        }
        net::GATEWAY_TYPE_GRAPHQL | _ => Box::new(net::GraphQlTransport::new(
            graphql_endpoint,
            GraphQlRequest::new(SCAFFOLD_QUERY),
        )),
    };
}

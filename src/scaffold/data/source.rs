use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{self, with_headers, GraphQlRequest, HttpClient};
use crate::common::result::NBResult;
use crate::config::ProjectConfigGatewayModel;
use crate::scaffold::data::dto::NativeScaffoldDataDto;
use crate::scaffold::data::mapper::to_model;
use crate::scaffold::data::query::SCAFFOLD_QUERY;
use crate::scaffold::domain::model::NativeScaffoldModel;

pub(super) async fn fetch_scaffold(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
) -> NBResult<NativeScaffoldModel> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(&gateway, graphql_endpoint);
    let dto: NativeScaffoldDataDto = net::request(http, headers, transport.as_ref()).await?;
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

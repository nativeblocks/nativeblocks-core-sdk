use std::collections::HashMap;

use serde_json::{Value, json};

use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{self, GatewayTransport, GraphQlRequest, HttpClient, with_headers};
use crate::common::result::NBResult;
use crate::config::ProjectConfigGatewayModel;
use crate::experiment::data::dto::NativeExperimentDataDto;
use crate::experiment::data::key;
use crate::experiment::data::mapper::to_model;
use crate::experiment::data::query::EXPERIMENT_QUERY;
use crate::experiment::domain::model::NativeExperimentModel;

pub(super) async fn fetch_experiment(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    key: &str,
    globals: &HashMap<String, String>,
) -> NBResult<NativeExperimentModel> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(&gateway, graphql_endpoint, key, globals);
    let dto: NativeExperimentDataDto = net::request(http, headers, transport.as_ref()).await?;
    return Ok(to_model(&dto));
}

fn build_transport(
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    key: &str,
    globals: &HashMap<String, String>,
) -> Box<dyn GatewayTransport> {
    return match gateway.gateway_type.as_str() {
        net::GATEWAY_TYPE_REST => {
            let mut variables = vec![(key::PARAM_KEY.to_string(), key.to_string())];
            if !globals.is_empty() {
                if let Ok(parameters) = serde_json::to_string(globals) {
                    variables.push((key::PARAM_PARAMETERS.to_string(), parameters));
                }
            }
            Box::new(net::RestTransport::new(gateway.value.clone(), variables))
        }
        net::GATEWAY_TYPE_GRAPHQL | _ => {
            let request =
                GraphQlRequest::new(EXPERIMENT_QUERY).with_variables(build_variables(key, globals));
            Box::new(net::GraphQlTransport::new(graphql_endpoint, request))
        }
    };
}

fn build_variables(key: &str, globals: &HashMap<String, String>) -> Value {
    let mut variables = json!({ "key": key });
    if !globals.is_empty() {
        let parameters: Vec<Value> = globals
            .iter()
            .map(|(k, v)| json!({ "key": k, "value": v }))
            .collect();
        variables["parameter"] = json!({ "variables": parameters });
    }
    return variables;
}

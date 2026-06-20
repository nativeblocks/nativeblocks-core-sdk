use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::net::{
    GraphQlRequest, HttpClient, auth_headers, decode_envelope, execute_graphql,
};
use crate::common::result::NbResult;
use crate::scaffold::data::dto::NativeScaffoldDataDto;
use crate::scaffold::key;
use crate::scaffold::model::NativeScaffoldModel;

#[derive(Debug, Clone)]
pub struct ScaffoldRequest {
    pub gateway: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub install_id: String,
}

#[async_trait]
pub trait Client: Send + Sync {
    async fn get_scaffold(&self, request: ScaffoldRequest) -> NbResult<NativeScaffoldModel>;
}

struct ClientImpl {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

pub fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
) -> Arc<dyn Client> {
    let logger = logger::get_or_create(environment.instance_name());
    Arc::new(ClientImpl {
        http,
        environment,
        config,
        logger,
    })
}

#[async_trait]
impl Client for ClientImpl {
    async fn get_scaffold(&self, request: ScaffoldRequest) -> NbResult<NativeScaffoldModel> {
        let mut headers = auth_headers(&self.environment, &self.config);
        headers.push((
            key::INSTALL_ID_HEADER.to_string(),
            request.install_id.clone(),
        ));

        let fetched: NbResult<NativeScaffoldDataDto> =
            if request.gateway.gateway_type == key::GATEWAY_TYPE_REST {
                self.http
                    .get(&request.gateway.value, &headers)
                    .await
                    .and_then(|body| decode_envelope::<NativeScaffoldDataDto>(&body))
            } else {
                let gql = GraphQlRequest::new(key::SCAFFOLD_QUERY);
                execute_graphql::<NativeScaffoldDataDto>(
                    self.http.as_ref(),
                    &request.graphql_endpoint,
                    &headers,
                    &gql,
                )
                .await
            };

        match fetched {
            Ok(dto) => {
                let scaffold = dto.to_model();
                self.log_success(scaffold.frames.len());
                Ok(scaffold)
            }
            Err(error) => {
                self.log_failure(&error);
                Err(error)
            }
        }
    }
}

impl ClientImpl {
    fn log_success(&self, frames_count: usize) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_SUCCEED.to_string(),
        );
        params.insert(
            keys::parameter::FRAMES_COUNT.to_string(),
            frames_count.to_string(),
        );
        self.dispatch(
            LoggerEventLevel::Info,
            "Successfully fetched scaffold",
            params,
        );
    }

    fn log_failure(&self, error: &crate::common::result::ErrorModel) {
        let mut params = error.to_logger_parameters();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_FAILED.to_string(),
        );
        self.dispatch(LoggerEventLevel::Error, "Failed to fetch scaffold", params);
    }

    fn dispatch(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(
                &self.config,
                level,
                keys::tag::SCAFFOLD_FETCH,
                message,
                params,
            );
        }
    }
}

#[cfg(test)]
#[path = "client.test.rs"]
mod tests;

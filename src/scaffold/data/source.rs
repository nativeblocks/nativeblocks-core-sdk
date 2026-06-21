use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, HttpClient, INSTALL_ID_HEADER, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::NBResult;
use crate::scaffold::data::dto::NativeScaffoldDataDto;
use crate::scaffold::graphql;
use crate::scaffold::model::ScaffoldRequest;

pub(crate) struct ScaffoldRemoteSource {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
}

impl ScaffoldRemoteSource {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        config: SdkConfig,
    ) -> Self {
        return Self {
            http,
            environment,
            config,
        };
    }

    pub(crate) async fn fetch(&self, request: &ScaffoldRequest) -> NBResult<NativeScaffoldDataDto> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), request.install_id.clone()));

        if request.gateway.gateway_type == GATEWAY_TYPE_REST {
            let body = self.http.get(&request.gateway.value, &headers).await?;
            return decode_envelope::<NativeScaffoldDataDto>(&body);
        }

        let gql = GraphQlRequest::new(graphql::SCAFFOLD_QUERY);
        return execute_graphql::<NativeScaffoldDataDto>(
            self.http.as_ref(),
            &request.graphql_endpoint,
            &headers,
            &gql,
        )
        .await;
    }
}

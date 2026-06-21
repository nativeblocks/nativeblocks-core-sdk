use crate::config::data::dto::ProjectConfigDataDto;
use crate::config::domain::key::GRAPHQL_GATEWAY_TYPE;
use crate::config::domain::model::NativeProjectConfigModel;

impl ProjectConfigDataDto {
    pub fn to_model(&self) -> NativeProjectConfigModel {
        let config = self.project_config.as_ref();
        return NativeProjectConfigModel {
            gateway: config
                .and_then(|c| c.gateway.clone())
                .unwrap_or_else(|| GRAPHQL_GATEWAY_TYPE.to_string()),
            endpoint: config.and_then(|c| c.endpoint.clone()).unwrap_or_default(),
            endpoints: config
                .and_then(|c| c.endpoints.clone())
                .unwrap_or_default(),
        };
    }
}

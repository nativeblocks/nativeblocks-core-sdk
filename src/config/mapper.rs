use crate::config::dto::ProjectConfigDataDto;
use crate::config::key::DEFAULT_GATEWAY_TYPE;
use crate::config::model::NativeProjectConfigModel;

pub(super) fn to_model(dto: &ProjectConfigDataDto) -> NativeProjectConfigModel {
    let inner = dto.project_config.as_ref();
    return NativeProjectConfigModel {
        gateway: inner
            .and_then(|c| c.gateway.clone())
            .unwrap_or_else(|| DEFAULT_GATEWAY_TYPE.to_string()),
        endpoint: inner.and_then(|c| c.endpoint.clone()).unwrap_or_default(),
        endpoints: inner.and_then(|c| c.endpoints.clone()).unwrap_or_default(),
    };
}

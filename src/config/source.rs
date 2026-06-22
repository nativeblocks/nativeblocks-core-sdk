use uuid::Uuid;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{HttpClient, map, with_headers};
use crate::common::result::{ErrorModel, NBResult};
use crate::config::dto::ProjectConfigDataDto;
use crate::config::key::{DEFAULT_GATEWAY_TYPE, INSTALL_ID_KEY, error_code, message};
use crate::config::mapper::to_model;
use crate::config::model::{NativeProjectConfigModel, ProjectConfigGatewayModel};

pub(crate) fn install_id(cache: &dyn CacheProvider) -> NBResult<String> {
    let existing = cache.get_string(INSTALL_ID_KEY.to_string(), String::new())?;
    if !existing.is_empty() {
        return Ok(existing);
    }
    let install_id = Uuid::now_v7().to_string();
    cache.save_string(INSTALL_ID_KEY.to_string(), install_id.clone(), None)?;
    return Ok(install_id);
}

pub(crate) async fn fetch_project_config(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    install_id: &str,
) -> NBResult<NativeProjectConfigModel> {
    let endpoint = environment.endpoint().ok_or_else(|| {
        ErrorModel::support(message::CLOUD_ONLY).with_code(error_code::PROJECT_CONFIG)
    })?;
    let headers = with_headers(environment, sdk_config, install_id);
    let body = http.get(endpoint.to_string(), headers).await?;
    let dto: ProjectConfigDataDto = map(&body)?;
    return Ok(to_model(&dto));
}

pub(crate) fn resolve_gateway(
    config: &NativeProjectConfigModel,
    operation: &str,
) -> ProjectConfigGatewayModel {
    return config
        .endpoints
        .iter()
        .find(|gateway| gateway.operation == operation)
        .cloned()
        .unwrap_or_else(|| ProjectConfigGatewayModel {
            operation: operation.to_string(),
            gateway_type: DEFAULT_GATEWAY_TYPE.to_string(),
            value: operation.to_string(),
        });
}

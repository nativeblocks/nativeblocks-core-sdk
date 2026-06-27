use uuid::Uuid;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::{HttpClient, map, with_headers};
use crate::common::result::NBResult;
use crate::config::dto::ProjectConfigDataDto;
use crate::config::key::{DEFAULT_GATEWAY_TYPE, INSTALL_ID_KEY};
use crate::config::mapper::to_model;
use crate::config::model::{NativeProjectConfigModel, ProjectConfigGatewayModel};

pub(crate) fn install_id(cache: &dyn CacheProvider) -> NBResult<String> {
    if let Some(bytes) = cache.get_bytes(INSTALL_ID_KEY.to_string())? {
        if let Ok(existing) = String::from_utf8(bytes) {
            if !existing.is_empty() {
                return Ok(existing);
            }
        }
    }
    let install_id = Uuid::now_v7().to_string();
    cache.save_bytes(INSTALL_ID_KEY.to_string(), install_id.clone().into_bytes(), None)?;
    return Ok(install_id);
}

pub(crate) async fn fetch_project_config(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    install_id: &str,
) -> NBResult<NativeProjectConfigModel> {
    let endpoint = environment.endpoint();
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

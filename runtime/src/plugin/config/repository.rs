use uuid::Uuid;

use crate::library::cache::util;
use crate::library::cache::{self, CacheProvider};
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::{HttpClient, map, with_headers};
use crate::library::result::NBResult;
use crate::plugin::config::dto::ProjectConfigDataDto;
use crate::plugin::config::key::{
    DEFAULT_GATEWAY_TYPE, INSTALL_ID_KEY, PROJECT_CONFIG_FRESH_KEY, PROJECT_CONFIG_KEY,
};
use crate::plugin::config::model::{NativeProjectConfigModel, ProjectConfigGatewayModel};

const PROJECT_CONFIG_TTL_MILLIS: i64 = 1 * 24 * 60 * 60 * 1000;

pub(crate) async fn install_id(cache: &dyn CacheProvider) -> NBResult<String> {
    if let Some(bytes) = cache.get(INSTALL_ID_KEY.to_string()).await? {
        if let Ok(existing) = String::from_utf8(bytes) {
            if !existing.is_empty() {
                return Ok(existing);
            }
        }
    }
    let install_id = Uuid::now_v7().to_string();
    cache
        .save(
            INSTALL_ID_KEY.to_string(),
            install_id.clone().into_bytes(),
            None,
        )
        .await?;
    return Ok(install_id);
}

pub(crate) async fn read_cached_config(
    cache: &dyn CacheProvider,
) -> NBResult<Option<NativeProjectConfigModel>> {
    return cache::read_or_cleanup(cache, PROJECT_CONFIG_KEY.to_string()).await;
}

pub(crate) async fn is_config_fresh(cache: &dyn CacheProvider) -> NBResult<bool> {
    return Ok(cache.has(PROJECT_CONFIG_FRESH_KEY.to_string()).await?);
}

pub(crate) async fn write_cached_config(
    cache: &dyn CacheProvider,
    config: &NativeProjectConfigModel,
) -> NBResult<()> {
    let bytes = util::to_bytes(config)?;
    cache.save(PROJECT_CONFIG_KEY.to_string(), bytes, None).await?;
    cache
        .save(
            PROJECT_CONFIG_FRESH_KEY.to_string(),
            b"1".to_vec(),
            Some(PROJECT_CONFIG_TTL_MILLIS),
        )
        .await?;
    return Ok(());
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

fn to_model(dto: &ProjectConfigDataDto) -> NativeProjectConfigModel {
    let inner = dto.project_config.as_ref();
    return NativeProjectConfigModel {
        gateway: inner
            .and_then(|c| c.gateway.clone())
            .unwrap_or_else(|| DEFAULT_GATEWAY_TYPE.to_string()),
        endpoint: inner.and_then(|c| c.endpoint.clone()).unwrap_or_default(),
        endpoints: inner.and_then(|c| c.endpoints.clone()).unwrap_or_default(),
    };
}

use serde_json::json;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::json;
use crate::common::net::{self, GatewayTransport, GraphQlRequest, HttpClient, with_headers};
use crate::common::result::{ErrorModel, NBResult};
use crate::config::ProjectConfigGatewayModel;
use crate::localization::data::dto::{
    NativeLocalizationDataDto, NativeLocalizationProductionChecksumDataDto,
};
use crate::localization::data::key::{self, error_code};
use crate::localization::data::mapper;
use crate::localization::data::query;
use crate::localization::domain::model::NativeLocalizationModel;

pub(super) async fn sync_cloud(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    localization_gateway: ProjectConfigGatewayModel,
    localization_production_gateway: ProjectConfigGatewayModel,
    localization_production_checksum_gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    if environment.development_mode() {
        return fetch_dev_localization(
            http,
            environment,
            sdk_config,
            cache,
            localization_gateway,
            graphql_endpoint,
            install_id,
            language_code,
        )
        .await;
    }

    return match cached_checksum(cache, language_code)? {
        None => {
            fetch_production_localization(
                http,
                environment,
                sdk_config,
                cache,
                localization_production_gateway,
                graphql_endpoint,
                install_id,
                language_code,
            )
            .await
        }
        Some(cached) => {
            let remote = fetch_production_checksum(
                http,
                environment,
                sdk_config,
                localization_production_checksum_gateway,
                graphql_endpoint,
                install_id,
                language_code,
            )
            .await?;
            if remote != cached {
                fetch_production_localization(
                    http,
                    environment,
                    sdk_config,
                    cache,
                    localization_production_gateway,
                    graphql_endpoint,
                    install_id,
                    language_code,
                )
                .await
            } else {
                get_localization(cache, language_code, false)
            }
        }
    };
}

pub(super) fn get_localization(
    cache: &dyn CacheProvider,
    language_code: &str,
    development_mode: bool,
) -> NBResult<NativeLocalizationModel> {
    let cache_key = if development_mode {
        key::dev_key(language_code)
    } else {
        key::prod_key(language_code)
    };
    return match cache.get_bytes(cache_key)? {
        Some(bytes) => decode_localization(&bytes),
        None => Err(ErrorModel::cache(key::message::LOCALIZATION_NOT_CACHED)
            .with_code(error_code::LOCALIZATION_NOT_CACHED)),
    };
}

async fn fetch_dev_localization(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    let localization = request_localization(
        http,
        environment,
        sdk_config,
        &gateway,
        graphql_endpoint,
        install_id,
        language_code,
        query::LOCALIZATION_QUERY,
        false,
    )
    .await
    .map_err(|error| error.or_code(error_code::LOCALIZATION_DEV_SYNC))?;
    cache_localization(cache, language_code, &localization, false)?;
    return Ok(localization);
}

async fn fetch_production_localization(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    cache: &dyn CacheProvider,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    language_code: &str,
) -> NBResult<NativeLocalizationModel> {
    let localization = request_localization(
        http,
        environment,
        sdk_config,
        &gateway,
        graphql_endpoint,
        install_id,
        language_code,
        query::LOCALIZATION_PRODUCTION_QUERY,
        true,
    )
    .await
    .map_err(|error| error.or_code(error_code::LOCALIZATION_PRODUCTION_SYNC))?;
    cache_localization(cache, language_code, &localization, true)?;
    return Ok(localization);
}

async fn fetch_production_checksum(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    language_code: &str,
) -> NBResult<String> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(
        &gateway,
        graphql_endpoint,
        language_code,
        query::LOCALIZATION_PRODUCTION_CHECKSUM_QUERY,
    );
    let data: NativeLocalizationProductionChecksumDataDto =
        net::request(http, headers, transport.as_ref())
            .await
            .map_err(|error| error.or_code(error_code::LOCALIZATION_CHECKSUM))?;
    return Ok(data
        .localization_production_checksum
        .and_then(|checksum| checksum.checksum)
        .unwrap_or_default());
}

fn cached_checksum(cache: &dyn CacheProvider, language_code: &str) -> NBResult<Option<String>> {
    return match cache.get_bytes(key::prod_key(language_code))? {
        Some(bytes) => Ok(decode_localization(&bytes)?.checksum),
        None => Ok(None),
    };
}

async fn request_localization(
    http: &dyn HttpClient,
    environment: &NativeblocksEnvironment,
    sdk_config: &SdkConfig,
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    install_id: &str,
    language_code: &str,
    query: &str,
    production: bool,
) -> NBResult<NativeLocalizationModel> {
    let headers = with_headers(environment, sdk_config, install_id);
    let transport = build_transport(gateway, graphql_endpoint, language_code, query);
    let data: NativeLocalizationDataDto = net::request(http, headers, transport.as_ref()).await?;
    let localizations = if production {
        data.localizations_production
    } else {
        data.localizations
    };
    return Ok(mapper::to_model(localizations.as_ref()));
}

fn build_transport(
    gateway: &ProjectConfigGatewayModel,
    graphql_endpoint: &str,
    language_code: &str,
    query: &str,
) -> Box<dyn GatewayTransport> {
    return match gateway.gateway_type.as_str() {
        net::GATEWAY_TYPE_REST => {
            let variables = vec![(
                key::PARAM_LANGUAGE_CODE.to_string(),
                language_code.to_string(),
            )];
            Box::new(net::RestTransport::new(gateway.value.clone(), variables))
        }
        net::GATEWAY_TYPE_GRAPHQL | _ => {
            let request = GraphQlRequest::new(query).with_variables(json!({ "languageCode": language_code }));
            Box::new(net::GraphQlTransport::new(graphql_endpoint, request))
        }
    };
}

fn cache_localization(
    cache: &dyn CacheProvider,
    language_code: &str,
    localization: &NativeLocalizationModel,
    production: bool,
) -> NBResult<()> {
    let cache_key = if production {
        key::prod_key(language_code)
    } else {
        key::dev_key(language_code)
    };
    let bytes = json::to_bytes(localization)?;
    cache.save_bytes(cache_key, bytes, None)?;
    return Ok(());
}

fn decode_localization(bytes: &[u8]) -> NBResult<NativeLocalizationModel> {
    return json::from_bytes(bytes)
        .map_err(|error| error.with_code(error_code::LOCALIZATION_NOT_CACHED));
}

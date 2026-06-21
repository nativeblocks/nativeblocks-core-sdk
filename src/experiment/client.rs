use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use serde_json::json;

use crate::common::cache::CacheProvider;
use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, HttpClient, INSTALL_ID_HEADER, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::{ErrorModel, NbResult};
use crate::experiment::data::dto::NativeExperimentDataDto;
use crate::experiment::graphql;
use crate::experiment::key::{CACHE_PREFIX, CACHE_SEPARATOR, message};
use crate::experiment::model::NativeExperimentModel;

#[derive(Debug, Clone)]
pub struct ExperimentRequest {
    pub gateway: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub install_id: String,
    pub key: String,
    pub parameters: HashMap<String, String>,
    pub cache_ttl_millis: i64,
}

#[async_trait]
pub trait Client: Send + Sync {
    async fn get_experiment(&self, request: ExperimentRequest) -> NbResult<NativeExperimentModel>;
}

struct ClientImpl {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    cache: Arc<dyn CacheProvider>,
}

pub fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> Arc<dyn Client> {
    let logger = logger::get_or_create(environment.instance_name());
    Arc::new(ClientImpl {
        http,
        environment,
        config,
        logger,
        cache,
    })
}

#[cfg(feature = "cache-sqlite")]
pub fn open_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    db_path: &str,
) -> NbResult<Arc<dyn Client>> {
    use crate::common::cache::sqlite::SqliteCacheProvider;
    let cache: Arc<dyn CacheProvider> = Arc::new(SqliteCacheProvider::open(db_path)?);
    Ok(new_client(http, environment, config, cache))
}

#[async_trait]
impl Client for ClientImpl {
    async fn get_experiment(
        &self,
        request: ExperimentRequest,
    ) -> NbResult<NativeExperimentModel> {
        if matches!(self.environment, NativeblocksEnvironment::Community { .. }) {
            let error = ErrorModel::support(message::CLOUD_REQUIRED);
            self.log_failure(&request.key, &error);
            return Err(error);
        }

        let cache_key = format!("{CACHE_PREFIX}{}", request.key);
        if self.cache.has(&cache_key).unwrap_or(false) {
            let cached = self.cache.get_string(&cache_key, "").unwrap_or_default();
            if !cached.is_empty() {
                let model = decode_cache_value(&cached);
                self.log_success(&request.key, &model);
                return Ok(model);
            }
        }

        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), request.install_id.clone()));

        let fetched: NbResult<NativeExperimentDataDto> =
            if request.gateway.gateway_type == GATEWAY_TYPE_REST {
                self.http
                    .get(&rest_url(&request), &headers)
                    .await
                    .and_then(|body| decode_envelope::<NativeExperimentDataDto>(&body))
            } else {
                let gql = GraphQlRequest::new(graphql::EXPERIMENT_QUERY)
                    .with_variables(graphql_variables(&request));
                execute_graphql::<NativeExperimentDataDto>(
                    self.http.as_ref(),
                    &request.graphql_endpoint,
                    &headers,
                    &gql,
                )
                .await
            };

        match fetched {
            Ok(dto) => {
                let model = dto.to_model();
                let ttl = (request.cache_ttl_millis > 0)
                    .then(|| Duration::from_millis(request.cache_ttl_millis as u64));
                let _ = self
                    .cache
                    .save_string(&cache_key, &encode_cache_value(&model), ttl);
                self.log_success(&request.key, &model);
                Ok(model)
            }
            Err(error) => {
                self.log_failure(&request.key, &error);
                Err(error)
            }
        }
    }
}

impl ClientImpl {
    fn log_success(&self, key: &str, model: &NativeExperimentModel) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::EXPERIMENT_FETCH_SUCCEED.to_string(),
        );
        params.insert(keys::parameter::EXPERIMENT_KEY.to_string(), key.to_string());
        params.insert(
            keys::parameter::EXPERIMENT_VALUE.to_string(),
            model.value.clone(),
        );
        params.insert(
            keys::parameter::EXPERIMENT_TYPE.to_string(),
            model.variable_type.clone(),
        );
        self.dispatch(
            LoggerEventLevel::Info,
            format!("Get experiment with key: {key}"),
            params,
        );
    }

    fn log_failure(&self, key: &str, error: &ErrorModel) {
        let mut params = error.to_logger_parameters();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::EXPERIMENT_FETCH_FAILED.to_string(),
        );
        params.insert(keys::parameter::EXPERIMENT_KEY.to_string(), key.to_string());
        self.dispatch(
            LoggerEventLevel::Error,
            format!("Get experiment failed for experiment: {key}"),
            params,
        );
    }

    fn dispatch(&self, level: LoggerEventLevel, message: String, params: HashMap<String, String>) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(
                &self.config,
                level,
                keys::tag::EXPERIMENT_STATE,
                message,
                params,
            );
        }
    }
}

fn graphql_variables(request: &ExperimentRequest) -> serde_json::Value {
    let variables: Vec<_> = request
        .parameters
        .iter()
        .map(|(k, v)| json!({ "key": k, "value": v }))
        .collect();
    json!({
        "key": request.key,
        "parameter": { "variables": variables },
    })
}

fn rest_url(request: &ExperimentRequest) -> String {
    let mut url = format!("{}?key={}", request.gateway.value, encode_query(&request.key));
    if !request.parameters.is_empty() {
        let parameters_json = serde_json::to_string(&request.parameters).unwrap_or_default();
        url.push_str(&format!("&parameters={}", encode_query(&parameters_json)));
    }
    url
}

fn encode_cache_value(model: &NativeExperimentModel) -> String {
    format!("{}{CACHE_SEPARATOR}{}", model.value, model.variable_type)
}

fn decode_cache_value(cached: &str) -> NativeExperimentModel {
    match cached.split_once(CACHE_SEPARATOR) {
        Some((value, variable_type)) => NativeExperimentModel {
            value: value.to_string(),
            variable_type: variable_type.to_string(),
        },
        None => NativeExperimentModel {
            value: String::new(),
            variable_type: String::new(),
        },
    }
}

fn encode_query(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
#[path = "client.test.rs"]
mod tests;

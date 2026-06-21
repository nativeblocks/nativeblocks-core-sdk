use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tokio::sync::watch;

use super::dto::{NativeFrameDataDto, NativeFrameProductionChecksumDataDto};
use super::mapper::frame_to_model;
use super::source::FrameLocalSource;
use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, HttpClient, INSTALL_ID_HEADER, Header, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::{ErrorModel, NbResult};
use crate::frame::graphql;
use crate::frame::key::{error_code, message};
use crate::frame::model::NativeFrameModel;

#[derive(Debug, Clone)]
pub(crate) struct FrameSyncRequest {
    pub endpoint_frame: ProjectConfigGateway,
    pub endpoint_frame_production: ProjectConfigGateway,
    pub endpoint_frame_production_checksum: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub development_mode: bool,
    pub install_id: String,
    pub route: String,
}

pub(crate) type FrameStream = watch::Receiver<NbResult<NativeFrameModel>>;

#[async_trait]
pub(crate) trait FrameRepository: Send + Sync {
    fn get_frame(&self, route: &str, development_mode: bool) -> FrameStream;
    async fn sync_frame(&self, request: FrameSyncRequest) -> NbResult<()>;
    async fn sync_community_frame(&self, endpoint_frame: &str, route: &str) -> NbResult<()>;
    fn clear_all_frames(&self) -> NbResult<()>;
    fn clear_frame(&self, route: &str) -> NbResult<()>;
    fn set_global_parameters(&self, parameters: HashMap<String, String>);
    fn get_global_parameters(&self) -> HashMap<String, String>;
}

pub(crate) fn new_frame_repository(
    http: Arc<dyn HttpClient>,
    source: Arc<dyn FrameLocalSource>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
) -> Arc<dyn FrameRepository> {
    Arc::new(FrameRepositoryImpl {
        http,
        source,
        environment,
        config,
        frame_cache: Mutex::new(HashMap::new()),
        streams: Mutex::new(HashMap::new()),
        global_parameters: Mutex::new(HashMap::new()),
    })
}

struct FrameRepositoryImpl {
    http: Arc<dyn HttpClient>,
    source: Arc<dyn FrameLocalSource>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    frame_cache: Mutex<HashMap<String, NativeFrameModel>>,
    streams: Mutex<HashMap<String, watch::Sender<NbResult<NativeFrameModel>>>>,
    global_parameters: Mutex<HashMap<String, String>>,
}

impl FrameRepositoryImpl {
    fn headers(&self, install_id: &str) -> Vec<Header> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        headers
    }

    fn load_from_cache(&self, route: &str, development_mode: bool) -> NbResult<NativeFrameModel> {
        if !development_mode {
            if let Some(model) = self.frame_cache.lock().unwrap().get(route).cloned() {
                return Ok(model);
            }
        }

        let json = if development_mode {
            self.source.dev_find_by_route(route)?
        } else {
            self.source.prod_find_by_route(route)?
        };

        match json {
            Some(json) if !json.is_empty() => {
                let model: NativeFrameModel = serde_json::from_str(&json)
                    .map_err(|e| ErrorModel::cache(e.to_string()).with_code(error_code::FRAME_CACHE_EMPTY))?;
                self.frame_cache
                    .lock()
                    .unwrap()
                    .insert(route.to_string(), model.clone());
                Ok(model)
            }
            _ => Err(ErrorModel::cache(message::CONNECTION_REQUIRED)
                .with_code(error_code::FRAME_CACHE_EMPTY)),
        }
    }

    fn publish(&self, route: &str, value: NbResult<NativeFrameModel>) {
        let mut streams = self.streams.lock().unwrap();
        match streams.get(route) {
            Some(sender) => {
                let _ = sender.send(value);
            }
            None => {
                let (sender, _) = watch::channel(value);
                streams.insert(route.to_string(), sender);
            }
        }
    }

    fn cache_and_publish(&self, route: &str, model: NativeFrameModel) {
        self.frame_cache
            .lock()
            .unwrap()
            .insert(route.to_string(), model.clone());
        self.publish(route, Ok(model));
    }

    async fn fetch_frame(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        route: &str,
        install_id: &str,
        production: bool,
    ) -> NbResult<NativeFrameModel> {
        let headers = self.headers(install_id);
        let parameters = self.get_global_parameters();

        let dto: NativeFrameDataDto = if gateway.gateway_type == GATEWAY_TYPE_REST {
            let url = rest_url(&gateway.value, route, &parameters);
            let body = self.http.get(&url, &headers).await?;
            decode_envelope::<NativeFrameDataDto>(&body)?
        } else {
            let query = if production {
                graphql::FRAME_PRODUCTION_QUERY
            } else {
                graphql::FRAME_QUERY
            };
            let request = GraphQlRequest::new(query)
                .with_variables(graphql::frame_variables(route, &parameters));
            execute_graphql::<NativeFrameDataDto>(
                self.http.as_ref(),
                graphql_endpoint,
                &headers,
                &request,
            )
            .await?
        };

        let model = if production {
            frame_to_model(dto.frame_production.as_ref())
        } else {
            frame_to_model(dto.frame.as_ref())
        };
        Ok(model)
    }

    async fn validate_checksum(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        install_id: &str,
        route: &str,
    ) -> NbResult<bool> {
        let headers = self.headers(install_id);
        let parameters = self.get_global_parameters();

        let dto: NativeFrameProductionChecksumDataDto =
            if gateway.gateway_type == GATEWAY_TYPE_REST {
                let url = rest_url(&gateway.value, route, &parameters);
                let body = self.http.get(&url, &headers).await?;
                decode_envelope::<NativeFrameProductionChecksumDataDto>(&body)?
            } else {
                let request = GraphQlRequest::new(graphql::FRAME_PRODUCTION_CHECKSUM_QUERY)
                    .with_variables(graphql::frame_variables(route, &parameters));
                execute_graphql::<NativeFrameProductionChecksumDataDto>(
                    self.http.as_ref(),
                    graphql_endpoint,
                    &headers,
                    &request,
                )
                .await?
            };

        let checksum = dto
            .frame_production_checksum
            .and_then(|c| c.checksum)
            .unwrap_or_default();
        let cached = self.source.prod_find_checksum(route, &checksum)?;
        Ok(checksum == cached.unwrap_or_default())
    }
}

#[async_trait]
impl FrameRepository for FrameRepositoryImpl {
    fn get_frame(&self, route: &str, development_mode: bool) -> FrameStream {
        if let Some(sender) = self.streams.lock().unwrap().get(route) {
            return sender.subscribe();
        }
        let value = self.load_from_cache(route, development_mode);
        let (sender, receiver) = watch::channel(value);
        self.streams
            .lock()
            .unwrap()
            .insert(route.to_string(), sender);
        receiver
    }

    async fn sync_frame(&self, request: FrameSyncRequest) -> NbResult<()> {
        let route = request.route.clone();
        if request.development_mode {
            let model = self
                .fetch_frame(
                    &request.endpoint_frame,
                    &request.graphql_endpoint,
                    &route,
                    &request.install_id,
                    false,
                )
                .await
                .map_err(|e| sync_error(e, error_code::FRAME_SYNC))?;
            let json = serialize(&model)?;
            self.source
                .dev_upsert(&route, model.checksum.as_deref().unwrap_or(""), &json)?;
            self.cache_and_publish(&route, model);
            return Ok(());
        }

        if !self.source.prod_exists(&route)? {
            let model = self
                .fetch_frame(
                    &request.endpoint_frame_production,
                    &request.graphql_endpoint,
                    &route,
                    &request.install_id,
                    true,
                )
                .await
                .map_err(|e| sync_error(e, error_code::FRAME_PRODUCTION_SYNC))?;
            let json = serialize(&model)?;
            self.source
                .prod_upsert(&route, model.checksum.as_deref().unwrap_or(""), &json)?;
            self.cache_and_publish(&route, model);
            return Ok(());
        }

        let valid = self
            .validate_checksum(
                &request.endpoint_frame_production_checksum,
                &request.graphql_endpoint,
                &request.install_id,
                &route,
            )
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_CHECKSUM))?;
        if valid {
            return Ok(());
        }

        let model = self
            .fetch_frame(
                &request.endpoint_frame_production,
                &request.graphql_endpoint,
                &route,
                &request.install_id,
                true,
            )
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_PRODUCTION_SYNC))?;
        let json = serialize(&model)?;
        self.source
            .prod_upsert(&route, model.checksum.as_deref().unwrap_or(""), &json)?;
        self.cache_and_publish(&route, model);
        Ok(())
    }

    async fn sync_community_frame(&self, endpoint_frame: &str, route: &str) -> NbResult<()> {
        let body = self
            .http
            .get(endpoint_frame, &[])
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_COMMUNITY_SYNC))?;
        let dto = decode_envelope::<NativeFrameDataDto>(&body)
            .map_err(|e| sync_error(e, error_code::FRAME_COMMUNITY_SYNC))?;
        let model = frame_to_model(dto.frame_production.as_ref());
        let json = serialize(&model)?;
        self.source
            .prod_upsert(route, model.checksum.as_deref().unwrap_or(""), &json)?;
        self.cache_and_publish(route, model);
        Ok(())
    }

    fn clear_all_frames(&self) -> NbResult<()> {
        self.source.clear_all()?;
        self.frame_cache.lock().unwrap().clear();
        Ok(())
    }

    fn clear_frame(&self, route: &str) -> NbResult<()> {
        self.source.clear(route)?;
        self.frame_cache.lock().unwrap().remove(route);
        Ok(())
    }

    fn set_global_parameters(&self, parameters: HashMap<String, String>) {
        *self.global_parameters.lock().unwrap() = parameters;
    }

    fn get_global_parameters(&self) -> HashMap<String, String> {
        self.global_parameters.lock().unwrap().clone()
    }
}

fn serialize(model: &NativeFrameModel) -> NbResult<String> {
    serde_json::to_string(model).map_err(|e| ErrorModel::cache(e.to_string()))
}

fn sync_error(error: ErrorModel, code: &str) -> ErrorModel {
    if error.error_code.is_some() {
        error
    } else {
        error.with_code(code)
    }
}

fn rest_url(base: &str, route: &str, parameters: &HashMap<String, String>) -> String {
    let separator = if base.contains('?') { '&' } else { '?' };
    let mut url = format!("{base}{separator}route={}", encode(route));
    if !parameters.is_empty() {
        let json = serde_json::to_string(parameters).unwrap_or_default();
        url.push_str(&format!("&parameters={}", encode(&json)));
    }
    url
}

fn encode(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

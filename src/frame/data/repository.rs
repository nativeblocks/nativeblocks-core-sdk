use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::result::{ErrorModel, NBResult};
use crate::frame::data::remote::FrameRemoteSource;
use crate::frame::data::source::FrameLocalSource;
use crate::frame::domain::key::{error_code, message};
use crate::frame::domain::model::{FrameSyncRequest, NativeFrameModel};

pub(crate) type FrameStream = watch::Receiver<NBResult<NativeFrameModel>>;

pub(crate) struct FrameRepository {
    remote: FrameRemoteSource,
    local: Arc<dyn FrameLocalSource>,
    frame_cache: Mutex<HashMap<String, NativeFrameModel>>,
    streams: Mutex<HashMap<String, watch::Sender<NBResult<NativeFrameModel>>>>,
    global_parameters: Mutex<HashMap<String, String>>,
}

impl FrameRepository {
    pub(crate) fn new(remote: FrameRemoteSource, local: Arc<dyn FrameLocalSource>) -> Self {
        return Self {
            remote,
            local,
            frame_cache: Mutex::new(HashMap::new()),
            streams: Mutex::new(HashMap::new()),
            global_parameters: Mutex::new(HashMap::new()),
        };
    }

    pub(crate) fn get_frame(&self, route: &str, development_mode: bool) -> FrameStream {
        if let Some(sender) = self.streams.lock().unwrap().get(route) {
            return sender.subscribe();
        }
        let value = self.load_from_cache(route, development_mode);
        let (sender, receiver) = watch::channel(value);
        self.streams
            .lock()
            .unwrap()
            .insert(route.to_string(), sender);
        return receiver;
    }

    pub(crate) async fn sync_frame(
        &self,
        request: &FrameSyncRequest,
        development_mode: bool,
    ) -> NBResult<()> {
        let route = &request.route;
        let parameters = self.get_global_parameters();

        if development_mode {
            let model = self
                .remote
                .fetch_frame(
                    &request.endpoint_frame,
                    &request.graphql_endpoint,
                    route,
                    &request.install_id,
                    false,
                    &parameters,
                )
                .await
                .map_err(|e| sync_error(e, error_code::FRAME_SYNC))?;
            let json = serialize(&model)?;
            self.local
                .dev_upsert(route, model.checksum.as_deref().unwrap_or(""), &json)?;
            self.cache_and_publish(route, model);
            return Ok(());
        }

        if !self.local.prod_exists(route)? {
            let model = self
                .remote
                .fetch_frame(
                    &request.endpoint_frame_production,
                    &request.graphql_endpoint,
                    route,
                    &request.install_id,
                    true,
                    &parameters,
                )
                .await
                .map_err(|e| sync_error(e, error_code::FRAME_PRODUCTION_SYNC))?;
            let json = serialize(&model)?;
            self.local
                .prod_upsert(route, model.checksum.as_deref().unwrap_or(""), &json)?;
            self.cache_and_publish(route, model);
            return Ok(());
        }

        let remote_checksum = self
            .remote
            .fetch_checksum(
                &request.endpoint_frame_production_checksum,
                &request.graphql_endpoint,
                route,
                &request.install_id,
                &parameters,
            )
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_CHECKSUM))?;
        let cached = self.local.prod_find_checksum(route, &remote_checksum)?;
        if remote_checksum == cached.unwrap_or_default() {
            return Ok(());
        }

        let model = self
            .remote
            .fetch_frame(
                &request.endpoint_frame_production,
                &request.graphql_endpoint,
                route,
                &request.install_id,
                true,
                &parameters,
            )
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_PRODUCTION_SYNC))?;
        let json = serialize(&model)?;
        self.local
            .prod_upsert(route, model.checksum.as_deref().unwrap_or(""), &json)?;
        self.cache_and_publish(route, model);
        return Ok(());
    }

    pub(crate) async fn sync_community_frame(
        &self,
        endpoint_frame: &str,
        route: &str,
    ) -> NBResult<()> {
        let model = self
            .remote
            .fetch_community(endpoint_frame)
            .await
            .map_err(|e| sync_error(e, error_code::FRAME_COMMUNITY_SYNC))?;
        let json = serialize(&model)?;
        self.local
            .prod_upsert(route, model.checksum.as_deref().unwrap_or(""), &json)?;
        self.cache_and_publish(route, model);
        return Ok(());
    }

    pub(crate) fn clear_all_frames(&self) -> NBResult<()> {
        self.local.clear_all()?;
        self.frame_cache.lock().unwrap().clear();
        return Ok(());
    }

    pub(crate) fn clear_frame(&self, route: &str) -> NBResult<()> {
        self.local.clear(route)?;
        self.frame_cache.lock().unwrap().remove(route);
        return Ok(());
    }

    pub(crate) fn set_global_parameters(&self, parameters: HashMap<String, String>) {
        *self.global_parameters.lock().unwrap() = parameters;
    }

    pub(crate) fn get_global_parameters(&self) -> HashMap<String, String> {
        return self.global_parameters.lock().unwrap().clone();
    }

    fn load_from_cache(&self, route: &str, development_mode: bool) -> NBResult<NativeFrameModel> {
        if !development_mode {
            if let Some(model) = self.frame_cache.lock().unwrap().get(route).cloned() {
                return Ok(model);
            }
        }

        let json = if development_mode {
            self.local.dev_find_by_route(route)?
        } else {
            self.local.prod_find_by_route(route)?
        };

        match json {
            Some(json) if !json.is_empty() => {
                let model: NativeFrameModel = serde_json::from_str(&json).map_err(|e| {
                    ErrorModel::cache(e.to_string()).with_code(error_code::FRAME_CACHE_EMPTY)
                })?;
                self.frame_cache
                    .lock()
                    .unwrap()
                    .insert(route.to_string(), model.clone());
                return Ok(model);
            }
            _ => {
                return Err(ErrorModel::cache(message::CONNECTION_REQUIRED)
                    .with_code(error_code::FRAME_CACHE_EMPTY));
            }
        }
    }

    fn publish(&self, route: &str, value: NBResult<NativeFrameModel>) {
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
}

fn serialize(model: &NativeFrameModel) -> NBResult<String> {
    return serde_json::to_string(model).map_err(|e| ErrorModel::cache(e.to_string()));
}

fn sync_error(error: ErrorModel, code: &str) -> ErrorModel {
    if error.error_code.is_some() {
        return error;
    }
    return error.with_code(code);
}

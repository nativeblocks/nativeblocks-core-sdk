use std::collections::HashMap;
use std::sync::Arc;

use crate::common::cache::new_cache_provider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::new_http_client;
use crate::common::result::NbError;
use crate::config;
use crate::frame::domain::action::NativeActionHandler;
use crate::frame::presenter::client::Client;
use crate::frame::di;
use crate::frame::domain::key::operation;
use crate::frame::domain::model::{
    FrameSyncRequest, NativeActionModel, NativeBlockModel, NativeFrameState, NativeVariableModel,
};
use crate::localization::LocalizationClient;

/// UniFFI handle for the frame engine. Mutating methods feed the headless
/// engine; the `*_state` getters return whole observable snapshots (the coarse
/// FFI projection of the engine's `watch` channels). The real GraphQL endpoint,
/// the frame gateways and the install id are resolved from the project config
/// (`config::Client`).
#[derive(uniffi::Object)]
pub struct FrameClient {
    inner: Client,
    config: Arc<config::Client>,
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        db_path: String,
        localization: Arc<LocalizationClient>,
    ) -> Result<Arc<Self>, NbError> {
        environment.validate()?;
        let http = new_http_client()?;
        let cache = new_cache_provider(&db_path)?;
        let config_client = config::get_or_create(http.clone(), &environment, &config, cache);
        let inner = di::open_engine(http, environment, config, &db_path, localization.inner())?;
        return Ok(Arc::new(Self {
            inner,
            config: config_client,
        }));
    }

    pub async fn setup_frame(&self, route: String, route_arguments: HashMap<String, String>) {
        self.inner.setup_frame(route, route_arguments).await;
    }

    pub async fn sync_frame(&self, route: String) -> Result<(), NbError> {
        let frame = self.config.gateway(operation::FRAME).await?;
        let production = self.config.gateway(operation::FRAME_PRODUCTION).await?;
        let checksum = self
            .config
            .gateway(operation::FRAME_PRODUCTION_CHECKSUM)
            .await?;
        let request = FrameSyncRequest {
            endpoint_frame: frame.gateway,
            endpoint_frame_production: production.gateway,
            endpoint_frame_production_checksum: checksum.gateway,
            graphql_endpoint: frame.endpoint,
            install_id: frame.install_id,
            route,
        };
        return self.inner.sync_frame(request).await.map_err(NbError::from);
    }

    pub async fn sync_community_frame(
        &self,
        endpoint_frame: String,
        route: String,
    ) -> Result<(), NbError> {
        return self
            .inner
            .sync_community_frame(endpoint_frame, route)
            .await
            .map_err(NbError::from);
    }

    pub fn handle_variable(&self, variable: NativeVariableModel, need_to_log: bool) {
        self.inner.handle_variable(variable, need_to_log);
    }

    pub async fn handle_action(
        &self,
        index: i32,
        action: Option<NativeActionModel>,
        performed_event_type: String,
    ) {
        self.inner
            .handle_action(index, action, performed_event_type)
            .await;
    }

    pub fn register_action_handler(
        &self,
        key_type: String,
        handler: Arc<dyn NativeActionHandler>,
    ) {
        self.inner.register_action_handler(key_type, handler);
    }

    pub fn change_block(&self, block: NativeBlockModel) {
        self.inner.change_block(block);
    }

    pub fn localize(&self, key: String) -> Option<String> {
        return self.inner.localize(key);
    }

    pub fn set_global_parameters(&self, parameters: HashMap<String, String>) {
        self.inner.set_global_parameters(parameters);
    }

    pub fn clear_all_frames(&self) -> Result<(), NbError> {
        return self.inner.clear_all_frames().map_err(NbError::from);
    }

    pub fn clear_frame(&self, route: String) -> Result<(), NbError> {
        return self.inner.clear_frame(route).map_err(NbError::from);
    }

    pub fn native_frame_state(&self) -> NativeFrameState {
        return self.inner.native_frame_state();
    }

    pub fn blocks_state(&self) -> HashMap<String, NativeBlockModel> {
        return self.inner.blocks_state();
    }

    pub fn variables_state(&self) -> HashMap<String, NativeVariableModel> {
        return self.inner.variables_state();
    }

    pub fn action_state(&self) -> HashMap<String, Vec<NativeActionModel>> {
        return self.inner.action_state();
    }

    pub fn frame_update_generation(&self) -> u32 {
        return self.inner.frame_update_generation();
    }
}

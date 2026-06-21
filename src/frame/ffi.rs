use std::collections::HashMap;
use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::net::HttpClient;
use crate::common::net::reqwest_client::ReqwestHttpClient;
use crate::common::result::NbError;
use crate::config;
use crate::frame::action::NativeActionHandler;
use crate::frame::client::{self, Client, FrameSyncRequest};
use crate::frame::key::operation;
use crate::frame::model::{
    NativeActionModel, NativeBlockModel, NativeFrameState, NativeVariableModel,
};
use crate::localization::LocalizationClient;

/// UniFFI handle for the frame engine. Mutating methods feed the headless
/// engine; the `*_state` getters return whole observable snapshots (the coarse
/// FFI projection of the engine's `watch` channels). The real GraphQL endpoint
/// and the frame gateways are resolved from the project config (`config::Client`).
#[derive(uniffi::Object)]
pub struct FrameClient {
    inner: Arc<dyn Client>,
    config: Arc<dyn config::Client>,
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
        let http: Arc<dyn HttpClient> = Arc::new(ReqwestHttpClient::new()?);
        let config_client = config::get_or_create(http.clone(), &environment, &config);
        let inner =
            client::open_engine(http, environment, config, &db_path, localization.inner())?;
        Ok(Arc::new(Self {
            inner,
            config: config_client,
        }))
    }

    pub async fn setup_frame(&self, route: String, route_arguments: HashMap<String, String>) {
        self.inner.setup_frame(route, route_arguments).await;
    }

    pub async fn sync_frame(&self, install_id: String, route: String) -> Result<(), NbError> {
        let project = self.config.project_config(&install_id).await?;
        let request = FrameSyncRequest {
            endpoint_frame: config::gateway_for(&project, operation::FRAME),
            endpoint_frame_production: config::gateway_for(&project, operation::FRAME_PRODUCTION),
            endpoint_frame_production_checksum: config::gateway_for(
                &project,
                operation::FRAME_PRODUCTION_CHECKSUM,
            ),
            graphql_endpoint: project.endpoint.clone(),
            install_id,
            route,
        };
        self.inner.sync_frame(request).await.map_err(NbError::from)
    }

    pub async fn sync_community_frame(
        &self,
        endpoint_frame: String,
        route: String,
    ) -> Result<(), NbError> {
        self.inner
            .sync_community_frame(endpoint_frame, route)
            .await
            .map_err(NbError::from)
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
        self.inner.localize(key)
    }

    pub fn set_global_parameters(&self, parameters: HashMap<String, String>) {
        self.inner.set_global_parameters(parameters);
    }

    pub fn clear_all_frames(&self) -> Result<(), NbError> {
        self.inner.clear_all_frames().map_err(NbError::from)
    }

    pub fn clear_frame(&self, route: String) -> Result<(), NbError> {
        self.inner.clear_frame(route).map_err(NbError::from)
    }

    pub fn native_frame_state(&self) -> NativeFrameState {
        self.inner.native_frame_state()
    }

    pub fn blocks_state(&self) -> HashMap<String, NativeBlockModel> {
        self.inner.blocks_state()
    }

    pub fn variables_state(&self) -> HashMap<String, NativeVariableModel> {
        self.inner.variables_state()
    }

    pub fn action_state(&self) -> HashMap<String, Vec<NativeActionModel>> {
        self.inner.action_state()
    }

    pub fn frame_update_generation(&self) -> u32 {
        self.inner.frame_update_generation()
    }
}

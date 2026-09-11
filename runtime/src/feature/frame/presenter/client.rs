use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::feature::frame::domain::repository::FrameRepository;
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::feature::frame::presenter::state_manager::{FrameSnapshot, FrameStateManager};
use crate::library::environment::model::SdkConfig;
use crate::library::result::NBError;
use crate::plugin::global_parameter::GlobalParameterProvider;
use crate::plugin::logger::NativeLoggerProvider;

#[derive(uniffi::Object)]
pub struct FrameClient {
    repository: Arc<dyn FrameRepository>,
    globals: Arc<GlobalParameterProvider>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    sdk_config: SdkConfig,
    snapshots: Arc<Mutex<HashMap<String, FrameSnapshot>>>,
}

impl FrameClient {
    pub(crate) fn create(
        repository: Arc<dyn FrameRepository>,
        globals: Arc<GlobalParameterProvider>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        sdk_config: SdkConfig,
    ) -> Arc<Self> {
        return Arc::new(Self {
            repository,
            globals,
            logger,
            sdk_config,
            snapshots: Arc::new(Mutex::new(HashMap::new())),
        });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameClient {
    pub fn state_manager(&self) -> Arc<FrameStateManager> {
        let logger = FrameLogger::new(self.logger.clone(), self.sdk_config.clone());
        return FrameStateManager::new(
            self.repository.clone(),
            self.globals.clone(),
            logger,
            self.snapshots.clone(),
        );
    }

    pub fn clear_frame_state(&self, state_key: String) {
        self.snapshots.lock().unwrap().remove(&state_key);
    }

    pub fn clear_all_frame_states(&self) {
        self.snapshots.lock().unwrap().clear();
    }

    pub async fn sync_frame(
        &self,
        route: String,
        parameters: HashMap<String, String>,
    ) -> Result<(), NBError> {
        return self
            .repository
            .sync(&route, &parameters)
            .await
            .map_err(NBError::from);
    }

    pub async fn clear(&self, route: String) -> Result<(), NBError> {
        return self.repository.clear(&route).await.map_err(NBError::from);
    }

    pub async fn clear_all(&self) -> Result<(), NBError> {
        return self.repository.clear_all().await.map_err(NBError::from);
    }
}

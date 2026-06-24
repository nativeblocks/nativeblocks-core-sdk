use std::collections::HashMap;
use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, keys};
use crate::common::net::HttpClient;
use crate::common::result::{ErrorModel, NbError};
use crate::frame::di::container::Container;
use crate::frame::domain::model::NativeFrameModel;
use crate::frame::presenter::global_parameter;
use crate::frame::presenter::state_manager::FrameStateManager;

#[derive(uniffi::Object)]
pub struct FrameClient {
    container: Arc<Container>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameClient {
    #[uniffi::constructor]
    pub fn new(
        environment: NativeblocksEnvironment,
        config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Result<Arc<Self>, NbError> {
        environment.validate()?;
        let container = Arc::new(Container::new(
            environment.clone(),
            config.clone(),
            http,
            cache,
        ));
        return Ok(Arc::new(Self {
            container,
            environment,
            sdk_config: config,
        }));
    }

    pub fn state_manager(&self) -> Arc<FrameStateManager> {
        return FrameStateManager::new(
            self.container.repository_arc(),
            self.environment.instance_name().to_string(),
            self.environment.development_mode(),
            self.sdk_config.clone(),
            logger::get_or_create(self.environment.instance_name()),
        );
    }

    /// Set the instance-wide global parameters merged into every frame's
    /// variables. Cloud-only — the community edition rejects this.
    pub fn set_global_parameters(
        &self,
        parameters: HashMap<String, String>,
    ) -> Result<(), NbError> {
        if self.environment.is_community() {
            let error =
                ErrorModel::support("To use GlobalParameters data, you need to use cloud edition");
            self.log_global_parameters(
                LoggerEventLevel::Error,
                &error.message,
                error.to_logger_parameters(),
            );
            return Err(NbError::from(error));
        }
        global_parameter::get_or_create(self.environment.instance_name()).set(parameters.clone());
        self.log_global_parameters(LoggerEventLevel::Info, "Global parameters set", parameters);
        return Ok(());
    }

    pub async fn sync_frame(
        &self,
        route: String,
        parameters: HashMap<String, String>,
    ) -> Result<NativeFrameModel, NbError> {
        return self
            .container
            .repository()
            .sync(&route, &parameters)
            .await
            .map_err(NbError::from);
    }

    pub fn get_frame(&self, route: String) -> Result<NativeFrameModel, NbError> {
        return self
            .container
            .repository()
            .get(&route)
            .map_err(NbError::from);
    }

    pub fn clear(&self, route: String) -> Result<(), NbError> {
        return self
            .container
            .repository()
            .clear(&route)
            .map_err(NbError::from);
    }

    pub fn clear_all(&self, routes: Vec<String>) -> Result<(), NbError> {
        return self
            .container
            .repository()
            .clear_all(&routes)
            .map_err(NbError::from);
    }
}

impl FrameClient {
    fn log_global_parameters(
        &self,
        level: LoggerEventLevel,
        message: &str,
        params: HashMap<String, String>,
    ) {
        if let Ok(provider) = logger::get_or_create(self.environment.instance_name()).lock() {
            provider.dispatch(
                &self.sdk_config,
                level,
                keys::tag::GLOBAL_PARAMETERS_CHANGE,
                message,
                params,
            );
        }
    }
}

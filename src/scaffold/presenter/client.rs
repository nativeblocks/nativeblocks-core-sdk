use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::config::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::{ErrorModel, NBResult};
use crate::scaffold::data::repository::ScaffoldRepository;
use crate::scaffold::domain::get_scaffold::get_scaffold_use_case;
use crate::scaffold::domain::model::{NativeScaffoldModel, ScaffoldRequest};

pub(crate) struct Client {
    repository: ScaffoldRepository,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    config: SdkConfig,
}

impl Client {
    pub(crate) fn new(
        repository: ScaffoldRepository,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        config: SdkConfig,
    ) -> Self {
        return Self {
            repository,
            logger,
            config,
        };
    }

    pub(crate) async fn get_scaffold(
        &self,
        request: ScaffoldRequest,
    ) -> NBResult<NativeScaffoldModel> {
        match get_scaffold_use_case(&self.repository, &request).await {
            Ok(scaffold) => {
                self.log_success(scaffold.frames.len());
                return Ok(scaffold);
            }
            Err(error) => {
                self.log_failure(&error);
                return Err(error);
            }
        }
    }

    fn log_success(&self, frames_count: usize) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_SUCCEED.to_string(),
        );
        params.insert(
            keys::parameter::FRAMES_COUNT.to_string(),
            frames_count.to_string(),
        );
        self.dispatch(LoggerEventLevel::Info, "Successfully fetched scaffold", params);
    }

    fn log_failure(&self, error: &ErrorModel) {
        let mut params = error.to_logger_parameters();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::SCAFFOLD_FETCH_FAILED.to_string(),
        );
        self.dispatch(LoggerEventLevel::Error, "Failed to fetch scaffold", params);
    }

    fn dispatch(&self, level: LoggerEventLevel, message: &str, params: HashMap<String, String>) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(
                &self.config,
                level,
                keys::tag::SCAFFOLD_FETCH,
                message,
                params,
            );
        }
    }
}

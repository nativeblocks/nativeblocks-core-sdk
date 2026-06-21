use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::config::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::{ErrorModel, NBResult};
use crate::experiment::data::repository::ExperimentRepository;
use crate::experiment::domain::get_experiment::get_experiment_use_case;
use crate::experiment::domain::model::{ExperimentRequest, NativeExperimentModel};

pub(crate) struct Client {
    repository: ExperimentRepository,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    config: SdkConfig,
}

impl Client {
    pub(crate) fn new(
        repository: ExperimentRepository,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        config: SdkConfig,
    ) -> Self {
        return Self {
            repository,
            logger,
            config,
        };
    }

    pub(crate) async fn get_experiment(
        &self,
        request: ExperimentRequest,
    ) -> NBResult<NativeExperimentModel> {
        match get_experiment_use_case(&self.repository, &request).await {
            Ok(model) => {
                self.log_success(&request.key, &model);
                return Ok(model);
            }
            Err(error) => {
                self.log_failure(&request.key, &error);
                return Err(error);
            }
        }
    }

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

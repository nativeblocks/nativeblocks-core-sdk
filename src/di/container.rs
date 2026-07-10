use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::{self, NativeLoggerProvider};
use crate::common::net::HttpClient;
use crate::config;
use crate::experiment::ExperimentRepository;
use crate::frame::FrameRepository;
use crate::global_parameter::GlobalParameterProvider;
use crate::localization::LocalizationRepository;
use crate::scaffold::ScaffoldRepository;

pub(crate) struct Container {
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    global_parameters: Arc<GlobalParameterProvider>,
    services: Mutex<Option<Arc<Services>>>,
}

impl Container {
    pub(crate) fn new(environment: NativeblocksEnvironment, sdk_config: SdkConfig) -> Self {
        let logger = logger::get_or_create(environment.instance_name());
        return Self {
            environment,
            sdk_config,
            logger,
            global_parameters: Arc::new(GlobalParameterProvider::new()),
            services: Mutex::new(None),
        };
    }

    pub(crate) fn environment(&self) -> &NativeblocksEnvironment {
        return &self.environment;
    }

    pub(crate) fn sdk_config(&self) -> &SdkConfig {
        return &self.sdk_config;
    }

    pub(crate) fn logger(&self) -> Arc<Mutex<NativeLoggerProvider>> {
        return self.logger.clone();
    }

    pub(crate) fn global_parameters(&self) -> Arc<GlobalParameterProvider> {
        return self.global_parameters.clone();
    }

    pub(crate) fn services(&self, http: Arc<dyn HttpClient>, cache: Arc<dyn CacheProvider>) -> Arc<Services> {
        let mut guard = self.services.lock().unwrap();
        if let Some(existing) = guard.as_ref() {
            return existing.clone();
        }
        let services = Arc::new(Services::build(self, http, cache));
        *guard = Some(services.clone());
        return services;
    }
}

pub(crate) struct Services {
    config_client: Arc<config::Client>,
    frame_repository: Arc<dyn FrameRepository>,
    localization_repository: Arc<dyn LocalizationRepository>,
    experiment_repository: Arc<dyn ExperimentRepository>,
    scaffold_repository: Arc<dyn ScaffoldRepository>,
}

impl Services {
    fn build(container: &Container, http: Arc<dyn HttpClient>, cache: Arc<dyn CacheProvider>) -> Self {
        let environment = container.environment.clone();
        let sdk_config = container.sdk_config.clone();
        let logger = container.logger.clone();

        let config_client = Arc::new(config::Client::new(
            http.clone(),
            environment.clone(),
            sdk_config.clone(),
            cache.clone(),
        ));

        let frame_repository = crate::frame::di::build_repository(
            environment.clone(),
            sdk_config.clone(),
            http.clone(),
            cache.clone(),
            config_client.clone(),
            logger.clone(),
        );
        let localization_repository = crate::localization::di::build_repository(
            environment.clone(),
            sdk_config.clone(),
            http.clone(),
            cache.clone(),
            config_client.clone(),
            logger.clone(),
        );
        let experiment_repository = crate::experiment::di::build_repository(
            environment.clone(),
            sdk_config.clone(),
            http.clone(),
            cache.clone(),
            config_client.clone(),
            logger.clone(),
        );
        let scaffold_repository = crate::scaffold::di::build_repository(
            environment,
            sdk_config,
            http,
            config_client.clone(),
            logger,
        );

        return Self {
            config_client,
            frame_repository,
            localization_repository,
            experiment_repository,
            scaffold_repository,
        };
    }

    #[allow(dead_code)]
    pub(crate) fn config_client(&self) -> Arc<config::Client> {
        return self.config_client.clone();
    }

    pub(crate) fn frame_repository(&self) -> Arc<dyn FrameRepository> {
        return self.frame_repository.clone();
    }

    pub(crate) fn localization_repository(&self) -> Arc<dyn LocalizationRepository> {
        return self.localization_repository.clone();
    }

    pub(crate) fn experiment_repository(&self) -> Arc<dyn ExperimentRepository> {
        return self.experiment_repository.clone();
    }

    pub(crate) fn scaffold_repository(&self) -> Arc<dyn ScaffoldRepository> {
        return self.scaffold_repository.clone();
    }
}

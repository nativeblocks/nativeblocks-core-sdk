use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::experiment::data::repository_impl::ExperimentRepositoryImpl;
use crate::experiment::domain::repository::ExperimentRepository;

pub(crate) struct Container {
    experiment_repository: Arc<dyn ExperimentRepository>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> Self {
        let experiment_repository = build_repository(environment, sdk_config, http, cache).unwrap();
        return Self {
            experiment_repository,
        };
    }

    pub(crate) fn repository(&self) -> &dyn ExperimentRepository {
        return self.experiment_repository.as_ref();
    }
}

fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<dyn ExperimentRepository>> {
    let logger = logger::get_or_create(environment.instance_name());
    let config_client =
        config::get_or_create(http.clone(), &environment, &sdk_config, cache.clone())?;
    return Ok(Arc::new(ExperimentRepositoryImpl::new(
        http,
        environment,
        sdk_config,
        cache,
        config_client,
        logger,
    )));
}

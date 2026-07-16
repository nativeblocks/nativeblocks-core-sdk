//! Factory for the experiment feature's repository. Called by the central DI
//! container (`di::Container`), which owns the built instance.

use std::sync::{Arc, Mutex};

use crate::common::cache::CacheProvider;
use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::config;
use crate::experiment::data::repository_impl::ExperimentRepositoryImpl;
use crate::experiment::domain::repository::ExperimentRepository;

pub(crate) fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
) -> Arc<dyn ExperimentRepository> {
    return Arc::new(ExperimentRepositoryImpl::new(
        http,
        environment,
        sdk_config,
        cache,
        config_client,
        logger,
    ));
}

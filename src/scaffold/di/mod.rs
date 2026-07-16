//! Factory for the scaffold feature's repository. Called by the central DI
//! container (`di::Container`), which owns the built instance.

use std::sync::{Arc, Mutex};

use crate::common::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::config;
use crate::scaffold::data::repository_impl::ScaffoldRepositoryImpl;
use crate::scaffold::domain::repository::ScaffoldRepository;

pub(crate) fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
) -> Arc<dyn ScaffoldRepository> {
    return Arc::new(ScaffoldRepositoryImpl::new(
        http,
        environment,
        sdk_config,
        config_client,
        logger,
    ));
}

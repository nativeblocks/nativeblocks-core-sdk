use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::experiment::presenter::client::Client;
use crate::experiment::data::repository::ExperimentRepository;
use crate::experiment::data::source::ExperimentRemoteSource;

pub(crate) fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
) -> Client {
    let logger = logger::get_or_create(environment.instance_name());
    let is_community = environment.is_community();
    let source = ExperimentRemoteSource::new(http, environment, config.clone());
    let repository = ExperimentRepository::new(source, cache, is_community);
    return Client::new(repository, logger, config);
}

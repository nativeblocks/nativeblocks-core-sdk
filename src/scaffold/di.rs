use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::scaffold::client::Client;
use crate::scaffold::data::repository::ScaffoldRepository;
use crate::scaffold::data::source::ScaffoldRemoteSource;

pub(crate) fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
) -> Client {
    let logger = logger::get_or_create(environment.instance_name());
    let source = ScaffoldRemoteSource::new(http, environment, config.clone());
    let repository = ScaffoldRepository::new(source);
    return Client::new(repository, logger, config);
}

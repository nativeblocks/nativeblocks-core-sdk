use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::localization::client::Client;
use crate::localization::data::remote::LocalizationRemoteSource;
use crate::localization::data::repository::LocalizationRepository;
use crate::localization::data::source::LocalizationLocalSource;

pub(crate) fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    local: Arc<dyn LocalizationLocalSource>,
) -> Client {
    let logger = logger::get_or_create(environment.instance_name());
    let development_mode = environment.development_mode();
    let is_community = environment.is_community();
    let remote = LocalizationRemoteSource::new(http, environment, config.clone());
    let repository = LocalizationRepository::new(remote, local, development_mode);
    return Client::new(repository, logger, config, is_community);
}

use std::sync::Arc;

use crate::common::config::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::frame::client::Client;
use crate::frame::data::remote::FrameRemoteSource;
use crate::frame::data::repository::FrameRepository;
use crate::frame::data::source::FrameLocalSource;
use crate::localization;

pub(crate) fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    frame_source: Arc<dyn FrameLocalSource>,
    localization: Arc<localization::Client>,
) -> Client {
    let development_mode = environment.development_mode();
    let instance_name = environment.instance_name().to_string();
    let logger = logger::get_or_create(environment.instance_name());
    let remote = FrameRemoteSource::new(http, environment, config.clone());
    let repository = FrameRepository::new(remote, frame_source);
    return Client::new(
        repository,
        localization,
        config,
        logger,
        development_mode,
        instance_name,
    );
}

#[cfg(feature = "cache-sqlite")]
pub(crate) fn open_engine(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    db_path: &str,
    localization: Arc<localization::Client>,
) -> NBResult<Client> {
    use crate::frame::data::source::new_frame_local_source;
    let frame_source = new_frame_local_source(db_path)?;
    return Ok(new_client(http, environment, config, frame_source, localization));
}

use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::frame::data::cloud_repository::CloudFrameRepository;
use crate::frame::domain::repository::FrameRepository;

pub(crate) struct Container {
    frame_repository: Arc<dyn FrameRepository>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> NBResult<Self> {
        let frame_repository = build_repository(environment, sdk_config, http, cache)?;
        return Ok(Self { frame_repository });
    }

    pub(crate) fn repository(&self) -> &dyn FrameRepository {
        return self.frame_repository.as_ref();
    }

    pub(crate) fn repository_arc(&self) -> Arc<dyn FrameRepository> {
        return self.frame_repository.clone();
    }
}

fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<dyn FrameRepository>> {
    let logger = logger::get_or_create(environment.instance_name());
    let config_client = config::get_or_create(http.clone(), &environment, &sdk_config, cache.clone())?;
    return Ok(Arc::new(CloudFrameRepository::new(
        http,
        environment,
        sdk_config,
        cache,
        config_client,
        logger,
    )));
}

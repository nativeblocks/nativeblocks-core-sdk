use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::localization::data::cloud_repository::CloudLocalizationRepository;
use crate::localization::domain::repository::LocalizationRepository;

pub(crate) struct Container {
    localization_repository: Arc<dyn LocalizationRepository>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> NBResult<Self> {
        let localization_repository =
            build_repository(environment, sdk_config, http, cache)?;
        return Ok(Self {
            localization_repository,
        });
    }

    pub(crate) fn repository(&self) -> &dyn LocalizationRepository {
        return self.localization_repository.as_ref();
    }

    pub(crate) fn repository_arc(&self) -> Arc<dyn LocalizationRepository> {
        return self.localization_repository.clone();
    }
}

fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<dyn LocalizationRepository>> {
    let logger = logger::get_or_create(environment.instance_name());
    let config_client =
        config::get_or_create(http.clone(), &environment, &sdk_config, cache.clone())?;
    return Ok(Arc::new(CloudLocalizationRepository::new(
        http,
        environment,
        sdk_config,
        cache,
        config_client,
        logger,
    )));
}

use std::sync::Arc;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::scaffold::data::repository_impl::ScaffoldRepositoryImpl;
use crate::scaffold::domain::repository::ScaffoldRepository;

pub(crate) struct Container {
    scaffold_repository: Arc<dyn ScaffoldRepository>,
}

impl Container {
    pub(crate) fn new(
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        http: Arc<dyn HttpClient>,
        cache: Arc<dyn CacheProvider>,
    ) -> NBResult<Self> {
        let scaffold_repository = build_repository(environment, sdk_config, http, cache)?;
        return Ok(Self {
            scaffold_repository,
        });
    }

    pub(crate) fn repository(&self) -> &dyn ScaffoldRepository {
        return self.scaffold_repository.as_ref();
    }
}

fn build_repository(
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    http: Arc<dyn HttpClient>,
    cache: Arc<dyn CacheProvider>,
) -> NBResult<Arc<dyn ScaffoldRepository>> {
    let logger = logger::get_or_create(environment.instance_name());
    let config_client =
        config::get_or_create(http.clone(), &environment, &sdk_config, cache.clone())?;
    return Ok(Arc::new(ScaffoldRepositoryImpl::new(
        http,
        environment,
        sdk_config,
        config_client,
        logger,
    )));
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::cache::{self, CacheProvider};
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::json;
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::experiment::data::key::{self, GATEWAY_OPERATION};
use crate::experiment::data::logging;
use crate::experiment::data::source;
use crate::experiment::domain::model::NativeExperimentModel;
use crate::experiment::domain::repository::ExperimentRepository;

pub(crate) struct ExperimentRepositoryImpl {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

impl ExperimentRepositoryImpl {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
        config_client: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Self {
        return Self {
            http,
            environment,
            sdk_config,
            cache,
            config_client,
            logger,
        };
    }

    async fn load(
        &self,
        key: &str,
        cache_ttl: Option<i64>,
        globals: &HashMap<String, String>,
    ) -> NBResult<NativeExperimentModel> {
        if let Some(cached) = self.cached(key)? {
            return Ok(cached);
        }
        let resolved = self.config_client.gateway(GATEWAY_OPERATION).await?;
        let experiment = source::fetch_experiment(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            resolved.gateway,
            &resolved.endpoint,
            &resolved.install_id,
            key,
            globals,
        )
        .await?;
        self.cache_experiment(key, &experiment, cache_ttl)?;
        return Ok(experiment);
    }

    fn cached(&self, key: &str) -> NBResult<Option<NativeExperimentModel>> {
        let cache_key = key::cache_key(key);
        if !self.cache.has(cache_key.clone())? {
            return Ok(None);
        }
        return cache::read_or_cleanup(self.cache.as_ref(), cache_key);
    }

    fn cache_experiment(
        &self,
        key: &str,
        experiment: &NativeExperimentModel,
        cache_ttl: Option<i64>,
    ) -> NBResult<()> {
        let bytes = json::to_bytes(experiment)?;
        self.cache.save_bytes(key::cache_key(key), bytes, cache_ttl)?;
        return Ok(());
    }
}

#[async_trait::async_trait]
impl ExperimentRepository for ExperimentRepositoryImpl {
    async fn fetch(
        &self,
        key: &str,
        cache_ttl: Option<i64>,
        globals: &HashMap<String, String>,
    ) -> NBResult<NativeExperimentModel> {
        let result = self.load(key, cache_ttl, globals).await;
        match &result {
            Ok(experiment) => {
                logging::log_success(&self.logger, &self.sdk_config, key, experiment)
            }
            Err(error) => logging::log_failure(&self.logger, &self.sdk_config, key, error),
        }
        return result;
    }
}

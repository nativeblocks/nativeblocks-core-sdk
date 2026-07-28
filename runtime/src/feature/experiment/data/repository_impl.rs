use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::feature::experiment::data::key::{CACHE_PREFIX, GATEWAY_OPERATION};
use crate::feature::experiment::data::logging;
use crate::feature::experiment::data::source;
use crate::feature::experiment::domain::model::NativeExperimentModel;
use crate::feature::experiment::domain::repository::ExperimentRepository;
use crate::library::cache::util;
use crate::library::cache::{self, CacheProvider};
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;
use crate::plugin::config;
use crate::plugin::logger::NativeLoggerProvider;

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
        if let Some(cached) = self.cached(key).await? {
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
        self.cache_experiment(key, &experiment, cache_ttl).await?;
        return Ok(experiment);
    }

    async fn cached(&self, key: &str) -> NBResult<Option<NativeExperimentModel>> {
        let cache_key = format_cache_key(key);
        if !self.cache.has(cache_key.clone()).await? {
            return Ok(None);
        }
        return cache::read_or_cleanup(self.cache.as_ref(), cache_key).await;
    }

    async fn cache_experiment(
        &self,
        key: &str,
        experiment: &NativeExperimentModel,
        cache_ttl: Option<i64>,
    ) -> NBResult<()> {
        let bytes = util::to_bytes(experiment)?;
        self.cache
            .save(format_cache_key(key), bytes, cache_ttl)
            .await?;
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
            Ok(experiment) => logging::log_success(&self.logger, &self.sdk_config, key, experiment),
            Err(error) => logging::log_failure(&self.logger, &self.sdk_config, key, error),
        }
        return result;
    }
}

fn format_cache_key(key: &str) -> String {
    return format!("{CACHE_PREFIX}{key}");
}

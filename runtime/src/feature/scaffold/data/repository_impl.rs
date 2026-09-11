use std::sync::{Arc, Mutex, Weak};

use crate::feature::scaffold::data::key::{self, GATEWAY_OPERATION};
use crate::feature::scaffold::data::logging;
use crate::feature::scaffold::data::source;
use crate::feature::scaffold::domain::model::ScaffoldModel;
use crate::feature::scaffold::domain::repository::ScaffoldRepository;
use crate::library::cache::util;
use crate::library::cache::{self, CacheProvider};
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::network::HttpClient;
use crate::library::result::NBResult;
use crate::plugin::config;
use crate::plugin::logger::NativeLoggerProvider;

pub(crate) struct ScaffoldRepositoryImpl {
    me: Weak<ScaffoldRepositoryImpl>,
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
}

impl ScaffoldRepositoryImpl {
    pub(crate) fn new(
        http: Arc<dyn HttpClient>,
        environment: NativeblocksEnvironment,
        sdk_config: SdkConfig,
        cache: Arc<dyn CacheProvider>,
        config_client: Arc<config::Client>,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            http,
            environment,
            sdk_config,
            cache,
            config_client,
            logger,
        });
    }

    async fn fetch_scaffold(&self) -> NBResult<ScaffoldModel> {
        let resolved = self.config_client.gateway(GATEWAY_OPERATION).await?;
        return source::fetch_scaffold(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            resolved.gateway,
            &resolved.endpoint,
            &resolved.install_id,
        )
        .await;
    }

    async fn fetch_and_cache(&self) -> NBResult<ScaffoldModel> {
        let result = self.fetch_scaffold().await;
        match &result {
            Ok(scaffold) => {
                logging::log_success(&self.logger, &self.sdk_config, scaffold.frames.len());
                self.cache_scaffold(scaffold).await;
            }
            Err(error) => logging::log_failure(&self.logger, &self.sdk_config, error),
        }
        return result;
    }

    async fn cached(&self) -> Option<ScaffoldModel> {
        return cache::read_or_cleanup(self.cache.as_ref(), self.cache_key())
            .await
            .ok()
            .flatten();
    }

    async fn cache_scaffold(&self, scaffold: &ScaffoldModel) {
        let Ok(bytes) = util::to_bytes(scaffold) else {
            return;
        };
        let _ = self.cache.save(self.cache_key(), bytes, None).await;
    }

    fn cache_key(&self) -> String {
        return key::cache_key(self.environment.development_mode()).to_string();
    }

    fn refresh_in_background(&self) {
        let Some(me) = self.me.upgrade() else {
            return;
        };
        tokio::spawn(async move {
            let _ = me.fetch_and_cache().await;
        });
    }
}

#[async_trait::async_trait]
impl ScaffoldRepository for ScaffoldRepositoryImpl {
    async fn fetch(&self, force_fetch: bool) -> NBResult<ScaffoldModel> {
        if !force_fetch {
            if let Some(scaffold) = self.cached().await {
                self.refresh_in_background();
                return Ok(scaffold);
            }
        }
        let result = self.fetch_and_cache().await;
        let Err(error) = &result else {
            return result;
        };
        if let Some(scaffold) = self.cached().await {
            return Ok(scaffold);
        }
        logging::log_not_cached(&self.logger, &self.sdk_config, error);
        return result;
    }

    async fn clear(&self) -> NBResult<()> {
        self.cache.clear().await?;
        return Ok(());
    }
}

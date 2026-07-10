use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::cache::CacheProvider;
use crate::common::environment::{NativeblocksEnvironment, SdkConfig};
use crate::common::logger::NativeLoggerProvider;
use crate::common::net::HttpClient;
use crate::common::result::NBResult;
use crate::config;
use crate::localization::data::key::{GATEWAY_LOCALIZATION, GATEWAY_LOCALIZATION_PRODUCTION, GATEWAY_LOCALIZATION_PRODUCTION_CHECKSUM};
use crate::localization::data::cloud_source;
use crate::localization::data::db_source;
use crate::localization::data::logging;
use crate::localization::data::memory_source::MemoryLocalizationSource;
use crate::localization::domain::model::NativeLocalizationModel;
use crate::localization::domain::repository::LocalizationRepository;

pub(crate) struct CloudLocalizationRepository {
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    sdk_config: SdkConfig,
    cache: Arc<dyn CacheProvider>,
    config_client: Arc<config::Client>,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    language_code: watch::Sender<Option<String>>,
    memory: MemoryLocalizationSource,
}

impl CloudLocalizationRepository {
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
            language_code: watch::channel(None).0,
            memory: MemoryLocalizationSource::new(),
        };
    }

    async fn fetch(&self, language_code: &str) -> NBResult<NativeLocalizationModel> {
        let localization = self.config_client.gateway(GATEWAY_LOCALIZATION).await?;
        let production = self
            .config_client
            .gateway(GATEWAY_LOCALIZATION_PRODUCTION)
            .await?;
        let checksum = self
            .config_client
            .gateway(GATEWAY_LOCALIZATION_PRODUCTION_CHECKSUM)
            .await?;
        return cloud_source::sync_cloud(
            self.http.as_ref(),
            &self.environment,
            &self.sdk_config,
            self.cache.as_ref(),
            localization.gateway,
            production.gateway,
            checksum.gateway,
            &localization.endpoint,
            &localization.install_id,
            language_code,
        )
        .await;
    }
}

#[async_trait::async_trait]
impl LocalizationRepository for CloudLocalizationRepository {
    async fn sync(&self, language_code: &str) -> NBResult<()> {
        let result = self.fetch(language_code).await;
        match &result {
            Ok(localization) => {
                self.memory.save_localization(language_code, localization.clone());
                logging::log_sync_success(&self.logger, &self.sdk_config, language_code);
            }
            Err(error) => {
                logging::log_sync_failure(&self.logger, &self.sdk_config, language_code, error)
            }
        }
        return result.map(|_| ());
    }

    async fn get(&self, language_code: &str) -> NBResult<NativeLocalizationModel> {
        let cached = self.memory.get_localization(language_code);
        if let Some(localization) = cached {
            return Ok(localization);
        }

        let from_db = db_source::get_localization(
            self.cache.as_ref(),
            language_code,
            self.environment.development_mode(),
        );
        match &from_db {
            Ok(localization) => {
                self.memory.save_localization(language_code, localization.clone());
                logging::log_load_success(&self.logger, &self.sdk_config, language_code);
            }
            Err(error) => {
                logging::log_load_failure(&self.logger, &self.sdk_config, language_code, error)
            }
        }
        return from_db;
    }

    fn set_language_code(&self, language_code: &str) {
        let changed = {
            let current = self.language_code.borrow();
            current.as_deref() != Some(language_code)
        };
        if changed {
            let _ = self.language_code.send(Some(language_code.to_string()));
            logging::log_language_set(&self.logger, &self.sdk_config, language_code);
        }
    }

    fn language_code(&self) -> watch::Receiver<Option<String>> {
        return self.language_code.subscribe();
    }

    fn translate(&self, key: &str) -> Option<String> {
        let language_code = self.language_code.borrow().clone();
        if language_code.is_none() {
            return None;
        }
        return self.memory.translate(&language_code.unwrap(), key);
    }
}

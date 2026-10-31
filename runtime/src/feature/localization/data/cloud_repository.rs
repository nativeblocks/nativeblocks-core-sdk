use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::feature::localization::data::db_source;
use crate::feature::localization::data::key::{
    GATEWAY_LOCALIZATION, GATEWAY_LOCALIZATION_PRODUCTION,
    GATEWAY_LOCALIZATION_PRODUCTION_CHECKSUM, error_code,
};
use crate::feature::localization::data::logging;
use crate::feature::localization::data::memory_source::MemoryLocalizationSource;
use crate::feature::localization::data::{binary_source, graphql_source};
use crate::feature::localization::domain::model::NativeLocalizationModel;
use crate::feature::localization::domain::repository::LocalizationRepository;
use crate::library::cache::CacheProvider;
use crate::library::environment::model::{NativeblocksEnvironment, SdkConfig};
use crate::library::net::{BinaryError, GatewayType, HttpClient, with_headers};
use crate::library::result::NBResult;
use crate::plugin::config::{self, ResolvedGatewayModel};
use crate::plugin::logger::NativeLoggerProvider;

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

    async fn from_cache(&self, language_code: &str) -> Option<NativeLocalizationModel> {
        if let Some(localization) = self.memory.get_localization(language_code) {
            return Some(localization);
        }
        let from_db = db_source::get_localization(
            self.cache.as_ref(),
            language_code,
            self.environment.development_mode(),
        )
        .await;
        match &from_db {
            Ok(localization) => {
                self.memory
                    .save_localization(language_code, localization.clone());
                logging::log_load_success(&self.logger, &self.sdk_config, language_code);
            }
            Err(error) => {
                logging::log_load_failure(&self.logger, &self.sdk_config, language_code, error)
            }
        }
        return from_db.ok();
    }

    async fn from_network(&self, language_code: &str) -> NBResult<NativeLocalizationModel> {
        let result = self.download_localization(language_code).await;
        match &result {
            Ok(localization) => {
                self.memory
                    .save_localization(language_code, localization.clone());
                logging::log_sync_success(&self.logger, &self.sdk_config, language_code);
            }
            Err(error) => {
                logging::log_sync_failure(&self.logger, &self.sdk_config, language_code, error)
            }
        }
        return result;
    }

    async fn download_localization(
        &self,
        language_code: &str,
    ) -> NBResult<NativeLocalizationModel> {
        if self.environment.development_mode() {
            let gateway = self.config_client.gateway(GATEWAY_LOCALIZATION).await?;
            let localization = self
                .development_localization(&gateway, language_code)
                .await
                .map_err(|error| error.or_code(error_code::LOCALIZATION_DEV_SYNC))?;
            db_source::save_localization(self.cache.as_ref(), language_code, &localization, false)
                .await?;
            return Ok(localization);
        }

        let gateway = self
            .config_client
            .gateway(GATEWAY_LOCALIZATION_PRODUCTION)
            .await?;
        let cached_checksum =
            db_source::cached_checksum(self.cache.as_ref(), language_code).await?;
        return match GatewayType::of(&gateway.gateway.gateway_type) {
            GatewayType::Binary => {
                self.binary_production_localization(&gateway, language_code, cached_checksum)
                    .await
            }
            GatewayType::GraphQl | GatewayType::Rest => {
                self.graphql_production_localization(&gateway, language_code, cached_checksum)
                    .await
            }
        };
    }

    async fn development_localization(
        &self,
        gateway: &ResolvedGatewayModel,
        language_code: &str,
    ) -> NBResult<NativeLocalizationModel> {
        return match GatewayType::of(&gateway.gateway.gateway_type) {
            GatewayType::Binary => {
                self.binary_development_localization(gateway, language_code)
                    .await
            }
            GatewayType::GraphQl | GatewayType::Rest => {
                self.graphql_development_localization(gateway, language_code)
                    .await
            }
        };
    }

    async fn binary_development_localization(
        &self,
        gateway: &ResolvedGatewayModel,
        language_code: &str,
    ) -> NBResult<NativeLocalizationModel> {
        let result = binary_source::localization(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.gateway.value,
            language_code,
        )
        .await;
        return match result {
            Ok(localization) => Ok(localization),
            Err(error) => {
                self.fallback(language_code, error)?;
                self.graphql_development_localization(gateway, language_code)
                    .await
            }
        };
    }

    async fn graphql_development_localization(
        &self,
        gateway: &ResolvedGatewayModel,
        language_code: &str,
    ) -> NBResult<NativeLocalizationModel> {
        return graphql_source::localization(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.endpoint,
            language_code,
        )
        .await;
    }

    async fn binary_production_localization(
        &self,
        gateway: &ResolvedGatewayModel,
        language_code: &str,
        cached_checksum: Option<String>,
    ) -> NBResult<NativeLocalizationModel> {
        let result = binary_source::localization_production(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.gateway.value,
            language_code,
            cached_checksum.as_deref(),
        )
        .await;
        return match result {
            Ok(Some(localization)) => {
                db_source::save_localization(
                    self.cache.as_ref(),
                    language_code,
                    &localization,
                    true,
                )
                .await?;
                Ok(localization)
            }
            Ok(None) => {
                db_source::get_localization(self.cache.as_ref(), language_code, false).await
            }
            Err(error) => {
                self.fallback(language_code, error)
                    .map_err(|error| error.or_code(error_code::LOCALIZATION_PRODUCTION_SYNC))?;
                self.graphql_production_localization(gateway, language_code, cached_checksum)
                    .await
            }
        };
    }

    async fn graphql_production_localization(
        &self,
        gateway: &ResolvedGatewayModel,
        language_code: &str,
        cached_checksum: Option<String>,
    ) -> NBResult<NativeLocalizationModel> {
        if let Some(cached) = cached_checksum {
            let checksum_gateway = self
                .config_client
                .gateway(GATEWAY_LOCALIZATION_PRODUCTION_CHECKSUM)
                .await?;
            let remote = graphql_source::localization_production_checksum(
                self.http.as_ref(),
                self.headers(&checksum_gateway),
                &checksum_gateway.endpoint,
                language_code,
            )
            .await
            .map_err(|error| error.or_code(error_code::LOCALIZATION_CHECKSUM))?;
            if remote == cached {
                return db_source::get_localization(self.cache.as_ref(), language_code, false)
                    .await;
            }
        }
        let localization = graphql_source::localization_production(
            self.http.as_ref(),
            self.headers(gateway),
            &gateway.endpoint,
            language_code,
        )
        .await
        .map_err(|error| error.or_code(error_code::LOCALIZATION_PRODUCTION_SYNC))?;
        db_source::save_localization(self.cache.as_ref(), language_code, &localization, true)
            .await?;
        return Ok(localization);
    }

    fn fallback(&self, language_code: &str, error: BinaryError) -> NBResult<()> {
        let error = match error {
            BinaryError::Failed(error) => return Err(error),
            BinaryError::Fallback(error) => error,
        };
        logging::log_binary_fallback(&self.logger, &self.sdk_config, language_code, &error);
        return Ok(());
    }

    fn headers(&self, gateway: &ResolvedGatewayModel) -> HashMap<String, String> {
        return with_headers(&self.environment, &self.sdk_config, &gateway.install_id);
    }
}

#[async_trait::async_trait]
impl LocalizationRepository for CloudLocalizationRepository {
    async fn load(&self, language_code: &str) -> NBResult<NativeLocalizationModel> {
        if let Some(localization) = self.from_cache(language_code).await {
            return Ok(localization);
        }
        return self.from_network(language_code).await;
    }

    async fn sync(&self, language_code: &str) -> NBResult<()> {
        return self.from_network(language_code).await.map(|_| ());
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
        let language_code = self.language_code.borrow().clone()?;
        return self.memory.translate(&language_code, key);
    }
}

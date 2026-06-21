use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use tokio::sync::watch;

use crate::common::config::{NativeblocksEnvironment, ProjectConfigGateway, SdkConfig};
use crate::common::logger::{self, LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::net::{
    GATEWAY_TYPE_REST, GraphQlRequest, HttpClient, INSTALL_ID_HEADER, Header, decode_envelope,
    execute_graphql, with_headers,
};
use crate::common::result::{ErrorModel, NbResult};
use crate::localization::data::dto::{
    NativeLocalizationDataDto, NativeLocalizationProductionChecksumDataDto,
};
use crate::localization::data::mapper::localization_to_model;
use crate::localization::data::source::LocalizationLocalSource;
use crate::localization::graphql;
use crate::localization::key::{error_code, message};
use crate::localization::model::NativeLocalizationModel;

#[derive(Debug, Clone)]
pub struct LocalizationSyncRequest {
    pub endpoint_localization: ProjectConfigGateway,
    pub endpoint_localization_production: ProjectConfigGateway,
    pub endpoint_localization_production_checksum: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub language_code: String,
    pub install_id: String,
}

#[async_trait]
pub trait Client: Send + Sync {
    async fn sync_localization(&self, request: LocalizationSyncRequest) -> NbResult<()>;
    async fn get_localization(&self, language_code: String) -> NbResult<()>;
    fn set_language_code(&self, language_code: String);
    fn get_language_code(&self) -> watch::Receiver<Option<String>>;
    fn translate(&self, key: String) -> Option<String>;
}

pub fn new_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    source: Arc<dyn LocalizationLocalSource>,
) -> Arc<dyn Client> {
    let development_mode = development_mode(&environment);
    let logger = logger::get_or_create(environment.instance_name());
    let (language_code, _) = watch::channel(None);
    Arc::new(ClientImpl {
        http,
        source,
        environment,
        config,
        logger,
        development_mode,
        language_code,
        localization_model: Mutex::new(None),
    })
}

#[cfg(feature = "cache-sqlite")]
pub fn open_client(
    http: Arc<dyn HttpClient>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    db_path: &str,
) -> NbResult<Arc<dyn Client>> {
    use crate::localization::data::source::sqlite::SqliteLocalizationDatabase;
    let source: Arc<dyn LocalizationLocalSource> = SqliteLocalizationDatabase::open(db_path)?;
    Ok(new_client(http, environment, config, source))
}

struct ClientImpl {
    http: Arc<dyn HttpClient>,
    source: Arc<dyn LocalizationLocalSource>,
    environment: NativeblocksEnvironment,
    config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    development_mode: bool,
    language_code: watch::Sender<Option<String>>,
    localization_model: Mutex<Option<NativeLocalizationModel>>,
}

impl ClientImpl {
    fn headers(&self, install_id: &str) -> Vec<Header> {
        let mut headers = with_headers(&self.environment, &self.config);
        headers.push((INSTALL_ID_HEADER.to_string(), install_id.to_string()));
        headers
    }

    async fn fetch(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        language_code: &str,
        install_id: &str,
        production: bool,
    ) -> NbResult<NativeLocalizationModel> {
        let headers = self.headers(install_id);

        let dto: NativeLocalizationDataDto = if gateway.gateway_type == GATEWAY_TYPE_REST {
            let url = format!("{}&languageCode={}", gateway.value, language_code);
            let body = self.http.get(&url, &headers).await?;
            decode_envelope::<NativeLocalizationDataDto>(&body)?
        } else {
            let query = if production {
                graphql::LOCALIZATIONS_PRODUCTION_QUERY
            } else {
                graphql::LOCALIZATIONS_QUERY
            };
            let request =
                GraphQlRequest::new(query).with_variables(graphql::variables(language_code));
            execute_graphql::<NativeLocalizationDataDto>(
                self.http.as_ref(),
                graphql_endpoint,
                &headers,
                &request,
            )
            .await?
        };

        let model = if production {
            localization_to_model(dto.localizations_production.as_ref())
        } else {
            localization_to_model(dto.localizations.as_ref())
        };
        Ok(model)
    }

    async fn validate_checksum(
        &self,
        gateway: &ProjectConfigGateway,
        graphql_endpoint: &str,
        language_code: &str,
        install_id: &str,
    ) -> NbResult<bool> {
        let headers = self.headers(install_id);

        let dto: NativeLocalizationProductionChecksumDataDto =
            if gateway.gateway_type == GATEWAY_TYPE_REST {
                let url = format!("{}&languageCode={}", gateway.value, language_code);
                let body = self.http.get(&url, &headers).await?;
                decode_envelope::<NativeLocalizationProductionChecksumDataDto>(&body)?
            } else {
                let request = GraphQlRequest::new(graphql::PRODUCTION_CHECKSUM_QUERY)
                    .with_variables(graphql::variables(language_code));
                execute_graphql::<NativeLocalizationProductionChecksumDataDto>(
                    self.http.as_ref(),
                    graphql_endpoint,
                    &headers,
                    &request,
                )
                .await?
            };

        let checksum = dto
            .localization_production_checksum
            .and_then(|c| c.checksum)
            .unwrap_or_default();
        let cached = self.source.prod_find_checksum(language_code, &checksum)?;
        Ok(checksum == cached.unwrap_or_default())
    }

    fn store_model(&self, model: NativeLocalizationModel) {
        *self.localization_model.lock().unwrap() = Some(model);
    }

    fn log(
        &self,
        level: LoggerEventLevel,
        event: &str,
        message: String,
        params: std::collections::HashMap<String, String>,
    ) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(&self.config, level, event, message, params);
        }
    }
}

#[async_trait]
impl Client for ClientImpl {
    async fn sync_localization(&self, request: LocalizationSyncRequest) -> NbResult<()> {
        if let NativeblocksEnvironment::Community { .. } = self.environment {
            let error = ErrorModel::support(message::CLOUD_ONLY);
            let mut params = error.to_logger_parameters();
            params.insert(
                keys::parameter::STATE.to_string(),
                keys::state::LOCALIZATION_SYNC_FAILED.to_string(),
            );
            self.log(
                LoggerEventLevel::Error,
                keys::tag::LOCALIZATION_SYNC_STATE,
                message::CLOUD_ONLY.to_string(),
                params,
            );
            return Err(error);
        }

        let lang = request.language_code.clone();
        let result = self.sync_remote(request, &lang).await;

        let mut params = std::collections::HashMap::new();
        let (level, state, message_text) = match &result {
            Ok(()) => (
                LoggerEventLevel::Info,
                keys::state::LOCALIZATION_SYNC_SUCCEED,
                format!("Localization synced for {lang}"),
            ),
            Err(error) => {
                params = error.to_logger_parameters();
                (
                    LoggerEventLevel::Error,
                    keys::state::LOCALIZATION_SYNC_FAILED,
                    format!("Localization sync failed for {lang}"),
                )
            }
        };
        params.insert(keys::parameter::STATE.to_string(), state.to_string());
        params.insert(keys::parameter::LANGUAGE_CODE.to_string(), lang);
        self.log(level, keys::tag::LOCALIZATION_SYNC_STATE, message_text, params);
        result
    }

    async fn get_localization(&self, language_code: String) -> NbResult<()> {
        let result = self.load(&language_code).await;

        let mut params = std::collections::HashMap::new();
        let (level, state, message_text) = match &result {
            Ok(()) => (
                LoggerEventLevel::Info,
                keys::state::LOCALIZATION_LOAD_SUCCEED,
                format!("Localization loaded: {language_code}"),
            ),
            Err(error) => {
                params = error.to_logger_parameters();
                (
                    LoggerEventLevel::Error,
                    keys::state::LOCALIZATION_LOAD_FAILED,
                    format!("Failed to load localization: {language_code}"),
                )
            }
        };
        params.insert(keys::parameter::STATE.to_string(), state.to_string());
        params.insert(
            keys::parameter::LANGUAGE_CODE.to_string(),
            language_code.clone(),
        );
        self.log(level, keys::tag::LOCALIZATION_STATE, message_text, params);
        result
    }

    fn set_language_code(&self, language_code: String) {
        let changed = self.language_code.borrow().as_deref() != Some(language_code.as_str());
        if changed {
            *self.localization_model.lock().unwrap() = None;
            self.language_code.send_replace(Some(language_code.clone()));
        }
        let mut params = std::collections::HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::LOCALIZATION_SET.to_string(),
        );
        params.insert(keys::parameter::LANGUAGE_CODE.to_string(), language_code);
        self.log(
            LoggerEventLevel::Info,
            keys::tag::LOCALIZATION_STATE,
            "Language changed".to_string(),
            params,
        );
    }

    fn get_language_code(&self) -> watch::Receiver<Option<String>> {
        self.language_code.subscribe()
    }

    fn translate(&self, key: String) -> Option<String> {
        self.localization_model
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|m| m.localizations.as_ref())
            .and_then(|map| map.get(&key).cloned())
    }
}

impl ClientImpl {
    async fn sync_remote(&self, request: LocalizationSyncRequest, lang: &str) -> NbResult<()> {
        if self.development_mode {
            let model = self
                .fetch(
                    &request.endpoint_localization,
                    &request.graphql_endpoint,
                    lang,
                    &request.install_id,
                    false,
                )
                .await
                .map_err(|e| with_code(e, error_code::SYNC))?;
            let json = serialize(&model)?;
            self.source
                .dev_upsert(lang, model.checksum.as_deref().unwrap_or(""), &json)?;
            self.store_model(model);
            return Ok(());
        }

        let valid = self
            .validate_checksum(
                &request.endpoint_localization_production_checksum,
                &request.graphql_endpoint,
                lang,
                &request.install_id,
            )
            .await
            .map_err(|e| with_code(e, error_code::CHECKSUM))?;
        if valid {
            return Ok(());
        }

        let model = self
            .fetch(
                &request.endpoint_localization_production,
                &request.graphql_endpoint,
                lang,
                &request.install_id,
                true,
            )
            .await
            .map_err(|e| with_code(e, error_code::PRODUCTION_SYNC))?;
        let json = serialize(&model)?;
        self.source
            .prod_upsert(lang, model.checksum.as_deref().unwrap_or(""), &json)?;
        self.store_model(model);
        Ok(())
    }

    async fn load(&self, language_code: &str) -> NbResult<()> {
        if self.localization_model.lock().unwrap().is_some() {
            return Ok(());
        }

        let json = if self.development_mode {
            self.source.dev_find_by_language_code(language_code)?
        } else {
            self.source.prod_find_by_language_code(language_code)?
        };

        match json {
            Some(json) if !json.is_empty() => {
                let model: NativeLocalizationModel = serde_json::from_str(&json)
                    .map_err(|e| ErrorModel::cache(e.to_string()).with_code(error_code::CACHE_EMPTY))?;
                self.store_model(model);
                Ok(())
            }
            _ => Err(ErrorModel::cache(message::CONNECTION_REQUIRED)
                .with_code(error_code::CACHE_EMPTY)),
        }
    }
}

fn serialize(model: &NativeLocalizationModel) -> NbResult<String> {
    serde_json::to_string(model).map_err(|e| ErrorModel::cache(e.to_string()))
}

fn with_code(error: ErrorModel, code: &str) -> ErrorModel {
    if error.error_code.is_some() {
        error
    } else {
        error.with_code(code)
    }
}

fn development_mode(environment: &NativeblocksEnvironment) -> bool {
    matches!(
        environment,
        NativeblocksEnvironment::Cloud {
            development_mode: true,
            ..
        }
    )
}

#[cfg(test)]
#[path = "client.test.rs"]
mod tests;

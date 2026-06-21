use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::config::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::{ErrorModel, NBResult};
use crate::localization::data::repository::LocalizationRepository;
use crate::localization::domain::get_localization::get_localization_use_case;
use crate::localization::domain::sync_localization::sync_localization_use_case;
use crate::localization::domain::key::message;
use crate::localization::domain::model::{LocalizationSyncRequest, NativeLocalizationModel};

pub(crate) struct Client {
    repository: LocalizationRepository,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    config: SdkConfig,
    is_community: bool,
    language_code: watch::Sender<Option<String>>,
    localization_model: Mutex<Option<NativeLocalizationModel>>,
}

impl Client {
    pub(crate) fn new(
        repository: LocalizationRepository,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        config: SdkConfig,
        is_community: bool,
    ) -> Self {
        let (language_code, _) = watch::channel(None);
        return Self {
            repository,
            logger,
            config,
            is_community,
            language_code,
            localization_model: Mutex::new(None),
        };
    }

    pub(crate) async fn sync_localization(
        &self,
        request: LocalizationSyncRequest,
    ) -> NBResult<()> {
        if self.is_community {
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
        let result = sync_localization_use_case(&self.repository, &request).await;
        if let Ok(Some(model)) = &result {
            *self.localization_model.lock().unwrap() = Some(model.clone());
        }

        let mut params = HashMap::new();
        let (level, state, message_text) = match &result {
            Ok(_) => (
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
        return result.map(|_| ());
    }

    pub(crate) async fn get_localization(&self, language_code: String) -> NBResult<()> {
        let result = self.ensure_loaded(&language_code).await;

        let mut params = HashMap::new();
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
        return result;
    }

    pub(crate) fn set_language_code(&self, language_code: String) {
        let changed = self.language_code.borrow().as_deref() != Some(language_code.as_str());
        if changed {
            *self.localization_model.lock().unwrap() = None;
            self.language_code.send_replace(Some(language_code.clone()));
        }
        let mut params = HashMap::new();
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

    pub(crate) fn get_language_code(&self) -> watch::Receiver<Option<String>> {
        return self.language_code.subscribe();
    }

    pub(crate) fn translate(&self, key: String) -> Option<String> {
        return self
            .localization_model
            .lock()
            .unwrap()
            .as_ref()
            .and_then(|m| m.localizations.as_ref())
            .and_then(|map| map.get(&key).cloned());
    }

    async fn ensure_loaded(&self, language_code: &str) -> NBResult<()> {
        let already_loaded = self.localization_model.lock().unwrap().is_some();
        if already_loaded {
            return Ok(());
        }
        let model = get_localization_use_case(&self.repository, language_code).await?;
        *self.localization_model.lock().unwrap() = Some(model);
        return Ok(());
    }

    fn log(
        &self,
        level: LoggerEventLevel,
        event: &str,
        message: String,
        params: HashMap<String, String>,
    ) {
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(&self.config, level, event, message, params);
        }
    }
}

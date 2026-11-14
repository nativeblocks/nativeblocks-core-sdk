use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::library::environment::model::SdkConfig;
use crate::library::result::ErrorModel;
use crate::plugin::logger::{self, LoggerEventLevel, NativeLoggerProvider, keys};

pub(super) struct SeedLog {
    logger: Arc<Mutex<NativeLoggerProvider>>,
    sdk_config: SdkConfig,
}

impl SeedLog {
    pub(super) fn new(logger: Arc<Mutex<NativeLoggerProvider>>, sdk_config: SdkConfig) -> Self {
        return Self { logger, sdk_config };
    }

    pub(super) fn seeded(&self, frames: usize, localizations: usize) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::FRAMES_COUNT.to_string(),
            frames.to_string(),
        );
        params.insert(
            keys::parameter::LOCALIZATIONS_COUNT.to_string(),
            localizations.to_string(),
        );
        self.dispatch(
            LoggerEventLevel::Debug,
            keys::state::EMBEDDED_SEEDED,
            "Copied embedded content into the cache",
            params,
        );
    }

    pub(super) fn skipped(&self, key: &str, reason: &str) {
        let mut params = HashMap::new();
        params.insert(keys::parameter::KEY.to_string(), key.to_string());
        params.insert(
            keys::parameter::ERROR_MESSAGE.to_string(),
            reason.to_string(),
        );
        self.dispatch(
            LoggerEventLevel::Error,
            keys::state::EMBEDDED_ENTRY_SKIPPED,
            "Skipped embedded content",
            params,
        );
    }

    pub(super) fn failed(&self, error: &ErrorModel) {
        self.dispatch(
            LoggerEventLevel::Error,
            keys::state::EMBEDDED_SEED_FAILED,
            "Failed to copy embedded content into the cache",
            logger::error_parameters(error),
        );
    }

    fn dispatch(
        &self,
        level: LoggerEventLevel,
        state: &str,
        message: &str,
        mut params: HashMap<String, String>,
    ) {
        params.insert(keys::parameter::STATE.to_string(), state.to_string());
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(
                &self.sdk_config,
                level,
                keys::tag::EMBEDDED_SEED,
                message,
                params,
            );
        }
    }
}

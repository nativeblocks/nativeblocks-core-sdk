use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::environment::model::SdkConfig;
use crate::common::logger::keys::parameter::{ERROR_MESSAGE, FRAME_ROUTE, STATE};
use crate::common::logger::keys::state::{FRAME_LOADING, FRAME_LOAD_FAILED, FRAME_LOAD_SUCCEED};
use crate::common::logger::keys::tag::FRAME_STATE;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider};
use crate::frame::presenter::state_manager::FrameState;

pub(crate) struct FrameLogger {
    provider: Arc<Mutex<NativeLoggerProvider>>,
    sdk_config: SdkConfig,
    route: Mutex<Option<String>>,
}

impl FrameLogger {
    pub(crate) fn new(provider: Arc<Mutex<NativeLoggerProvider>>, sdk_config: SdkConfig) -> Self {
        return Self {
            provider,
            sdk_config,
            route: Mutex::new(None),
        };
    }

    fn enabled(&self) -> bool {
        if let Ok(provider) = self.provider.lock() {
            return provider.has_loggers();
        }
        return false;
    }

    pub(crate) fn set_route(&self, route: &str) {
        *self.route.lock().unwrap() = Some(route.to_string());
    }

    pub(crate) fn frame_state_changed(&self, frame_state: &FrameState) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        match frame_state {
            FrameState::Loading {} => {
                params.insert(STATE.to_string(), FRAME_LOADING.to_string());
                self.dispatch(
                    LoggerEventLevel::Debug,
                    FRAME_STATE,
                    "Frame state: loading".to_string(),
                    params,
                );
            }
            FrameState::Ready {} => {
                params.insert(STATE.to_string(), FRAME_LOAD_SUCCEED.to_string());
                self.dispatch(
                    LoggerEventLevel::Info,
                    FRAME_STATE,
                    "Frame state: ready".to_string(),
                    params,
                );
            }
            FrameState::Error { message } => {
                params.insert(STATE.to_string(), FRAME_LOAD_FAILED.to_string());
                params.insert(ERROR_MESSAGE.to_string(), message.clone());
                self.dispatch(
                    LoggerEventLevel::Error,
                    FRAME_STATE,
                    format!("Frame state: error - {message}"),
                    params,
                );
            }
        }
    }

    fn dispatch(
        &self,
        level: LoggerEventLevel,
        tag: &str,
        message: String,
        mut params: HashMap<String, String>,
    ) {
        params.insert(
            FRAME_ROUTE.to_string(),
            self.route.lock().unwrap().clone().unwrap_or_default(),
        );
        if let Ok(provider) = self.provider.lock() {
            provider.dispatch(&self.sdk_config, level, tag, message, params);
        }
    }
}

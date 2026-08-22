use crate::feature::frame::presenter::state_manager::model::{ActionLogEvent, RenderingState};
use crate::library::environment::model::SdkConfig;
use crate::plugin::logger::keys::parameter::{
    ACTION_NAME, ERROR_MESSAGE, EVENT_NAME, FRAME_ROUTE, KEY_TYPE, STATE, THEN, TRIGGER_NAME,
};
use crate::plugin::logger::keys::state::{
    ACTION_EVENT_IGNORED, ACTION_EVENT_TRIGGERED, FALLBACK_TRIGGER, FRAME_LOAD_FAILED,
    FRAME_LOAD_SUCCEED, FRAME_LOADING, TRIGGER_EXECUTED,
};
use crate::plugin::logger::keys::tag::{FALLBACK_ACTION, FRAME_STATE, HANDLE_ACTION};
use crate::plugin::logger::{LoggerEventLevel, NativeLoggerProvider};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

    pub(crate) fn frame_state_changed(&self, frame_state: &RenderingState) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        match frame_state {
            RenderingState::Loading {} => {
                params.insert(STATE.to_string(), FRAME_LOADING.to_string());
                self.dispatch(
                    LoggerEventLevel::Debug,
                    FRAME_STATE,
                    "Frame state: loading".to_string(),
                    params,
                );
            }
            RenderingState::Ready {} => {
                params.insert(STATE.to_string(), FRAME_LOAD_SUCCEED.to_string());
                self.dispatch(
                    LoggerEventLevel::Debug,
                    FRAME_STATE,
                    "Frame state: ready".to_string(),
                    params,
                );
            }
            RenderingState::Error { message } => {
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

    pub(crate) fn action(&self, event: ActionLogEvent) {
        if !self.enabled() {
            return;
        }
        match event {
            ActionLogEvent::EventIgnored { event } => self.dispatch(
                LoggerEventLevel::Debug,
                HANDLE_ACTION,
                format!("No action bound to '{event}'"),
                HashMap::from([
                    (STATE.to_string(), ACTION_EVENT_IGNORED.to_string()),
                    (EVENT_NAME.to_string(), event),
                ]),
            ),
            ActionLogEvent::EventTriggered { event, action_key } => self.dispatch(
                LoggerEventLevel::Debug,
                HANDLE_ACTION,
                format!("Event '{event}' handled"),
                HashMap::from([
                    (STATE.to_string(), ACTION_EVENT_TRIGGERED.to_string()),
                    (EVENT_NAME.to_string(), event),
                    (ACTION_NAME.to_string(), action_key),
                ]),
            ),
            ActionLogEvent::TriggerExecuted {
                name,
                key_type,
                then,
            } => self.dispatch(
                LoggerEventLevel::Debug,
                HANDLE_ACTION,
                format!("Trigger '{name}' executed"),
                HashMap::from([
                    (STATE.to_string(), TRIGGER_EXECUTED.to_string()),
                    (TRIGGER_NAME.to_string(), name),
                    (KEY_TYPE.to_string(), key_type),
                    (THEN.to_string(), then),
                ]),
            ),
            ActionLogEvent::TriggerFallback { key_type, name } => self.dispatch(
                LoggerEventLevel::Error,
                FALLBACK_ACTION,
                format!("No action registered for '{key_type}'"),
                HashMap::from([
                    (STATE.to_string(), FALLBACK_TRIGGER.to_string()),
                    (KEY_TYPE.to_string(), key_type),
                    (TRIGGER_NAME.to_string(), name),
                ]),
            ),
        }
    }

    pub(crate) fn variable_changed(&self, key: &str, dirty_count: usize) {
        if !self.enabled() {
            return;
        }
        self.dispatch(
            LoggerEventLevel::Debug,
            FRAME_STATE,
            format!("Variable '{key}' changed -> {dirty_count} block(s) invalidated"),
            HashMap::new(),
        );
    }

    pub(crate) fn injected_variable_write(&self, key: &str) {
        if !self.enabled() {
            return;
        }
        self.dispatch(
            LoggerEventLevel::Debug,
            FRAME_STATE,
            format!("Variable '{key}' is a global or route argument and can not be updated"),
            HashMap::new(),
        );
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

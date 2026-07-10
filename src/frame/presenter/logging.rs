use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::common::environment::SdkConfig;
use crate::common::logger::keys::parameter::{ACTION_NAME, BLOCK_KEY, ERROR_MESSAGE, EVENT_NAME, FRAME_ROUTE, KEY, KEY_TYPE, NEW_VALUE, PREVIOUS_VALUE, STATE, THEN, TRIGGER_NAME, VARIABLE_TYPE};
use crate::common::logger::keys::state::{ACTION_EVENT_IGNORED, ACTION_EVENT_TRIGGERED, BLOCK_UPDATED, FALLBACK_TRIGGER, FRAME_LOADING, FRAME_LOAD_FAILED, FRAME_LOAD_SUCCEED, TRIGGER_EXECUTED, VARIABLE_UPDATED};
use crate::common::logger::keys::tag::{BLOCK_CHANGE, FALLBACK_ACTION, FRAME_STATE, HANDLE_ACTION, VARIABLE_CHANGE};
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider};
use crate::frame::domain::model::{NativeActionTriggerModel, NativeActionTriggerThen};
use crate::frame::presenter::state_manager::FrameState;

pub(crate) struct FrameLogger {
    provider: Arc<Mutex<NativeLoggerProvider>>,
    sdk_config: SdkConfig,
    development_mode: bool,
    route: Mutex<Option<String>>,
}

impl FrameLogger {
    pub(crate) fn new(
        provider: Arc<Mutex<NativeLoggerProvider>>,
        sdk_config: SdkConfig,
        development_mode: bool,
    ) -> Self {
        return Self {
            provider,
            sdk_config,
            development_mode,
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

    fn change_level(&self) -> LoggerEventLevel {
        return if self.development_mode {
            LoggerEventLevel::Debug
        } else {
            LoggerEventLevel::Info
        };
    }

    pub(crate) fn variable_changed(
        &self,
        previous: Option<&str>,
        key: &str,
        value: &str,
        variable_type: &str,
    ) {
        if !self.enabled() {
            return;
        }
        let changed = previous.is_some_and(|previous| previous != value);
        let message = match previous.filter(|_| changed) {
            Some(previous) => {
                format!("Variable changed: {key} changed from '{previous}' to '{value}'")
            }
            None => format!("Variable changed: {key} set to '{value}'"),
        };
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), VARIABLE_UPDATED.to_string());
        params.insert(KEY.to_string(), key.to_string());
        params.insert(NEW_VALUE.to_string(), value.to_string());
        params.insert(VARIABLE_TYPE.to_string(), variable_type.to_string());
        if changed {
            params.insert(PREVIOUS_VALUE.to_string(), previous.unwrap_or_default().to_string());
        }
        self.dispatch(self.change_level(), VARIABLE_CHANGE, message, params);
    }

    pub(crate) fn block_changed(&self, key: &str, key_type: &str) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), BLOCK_UPDATED.to_string());
        params.insert(BLOCK_KEY.to_string(), key.to_string());
        params.insert(KEY_TYPE.to_string(), key_type.to_string());
        self.dispatch(self.change_level(), BLOCK_CHANGE, format!("Block updated: {key}[{key_type}]"), params);
    }

    pub(crate) fn action_triggered(&self, key: &str, event: &str) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), ACTION_EVENT_TRIGGERED.to_string());
        params.insert(EVENT_NAME.to_string(), event.to_string());
        params.insert(KEY.to_string(), key.to_string());
        self.dispatch(LoggerEventLevel::Info, HANDLE_ACTION, format!("Action event triggered: {event} for {key}"), params);
    }

    pub(crate) fn action_ignored(&self, key: &str, event: &str) {
        if !self.enabled() || !self.development_mode {
            return;
        }
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), ACTION_EVENT_IGNORED.to_string());
        params.insert(EVENT_NAME.to_string(), event.to_string());
        params.insert(KEY.to_string(), key.to_string());
        self.dispatch(LoggerEventLevel::Debug, HANDLE_ACTION, format!("Action event ignored: no {event} action for {key}"), params);
    }

    pub(crate) fn trigger_executed(&self, action_label: &str, trigger: &NativeActionTriggerModel) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), TRIGGER_EXECUTED.to_string());
        params.insert(ACTION_NAME.to_string(), action_label.to_string());
        params.insert(TRIGGER_NAME.to_string(), trigger.name.clone());
        params.insert(KEY_TYPE.to_string(), trigger.key_type.clone());
        params.insert(THEN.to_string(), then_name(trigger.then).to_string());
        self.dispatch(LoggerEventLevel::Info, HANDLE_ACTION, format!("Executing trigger: {} [{}]", trigger.name, trigger.key_type), params);
    }

    pub(crate) fn action_fallback(&self, key_type: &str, name: &str) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        params.insert(STATE.to_string(), FALLBACK_TRIGGER.to_string());
        params.insert(KEY_TYPE.to_string(), key_type.to_string());
        params.insert(ACTION_NAME.to_string(), name.to_string());
        self.dispatch(LoggerEventLevel::Warning, FALLBACK_ACTION, format!("Fallback action triggered: {name} is not available"), params);
    }

    pub(crate) fn frame_state_changed(&self, frame_state: &FrameState) {
        if !self.enabled() {
            return;
        }
        let mut params = HashMap::new();
        match frame_state {
            FrameState::Loading {} => {
                params.insert(STATE.to_string(), FRAME_LOADING.to_string());
                self.dispatch(LoggerEventLevel::Debug, FRAME_STATE, "Frame state: loading".to_string(), params);
            }
            FrameState::Ready {} => {
                params.insert(STATE.to_string(), FRAME_LOAD_SUCCEED.to_string());
                self.dispatch(LoggerEventLevel::Info, FRAME_STATE, "Frame state: ready".to_string(), params);
            }
            FrameState::Error { message } => {
                params.insert(STATE.to_string(), FRAME_LOAD_FAILED.to_string());
                params.insert(ERROR_MESSAGE.to_string(), message.clone());
                self.dispatch(LoggerEventLevel::Error, FRAME_STATE, format!("Frame state: error - {message}"), params);
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
        params.insert(FRAME_ROUTE.to_string(), self.route.lock().unwrap().clone().unwrap_or_default());
        if let Ok(provider) = self.provider.lock() {
            provider.dispatch(&self.sdk_config, level, tag, message, params);
        }
    }
}

fn then_name(then: NativeActionTriggerThen) -> &'static str {
    return match then {
        NativeActionTriggerThen::Success => "SUCCESS",
        NativeActionTriggerThen::Failure => "FAILURE",
        NativeActionTriggerThen::Next => "NEXT",
        NativeActionTriggerThen::End => "END",
    };
}

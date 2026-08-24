use crate::feature::frame::presenter::state_manager::model::{
    ActionLogEvent, BlockLogEvent, RenderingState,
};
use crate::library::environment::model::SdkConfig;
use crate::plugin::logger::keys::parameter::{
    ACTION_NAME, BLOCK_KEY, ERROR_MESSAGE, EVENT_NAME, FRAME_ROUTE, KEY, KEY_TYPE, MATCH_COUNT,
    PROVIDED_SCOPE, REQUIRED_SCOPE, STATE, TRIGGER_NAME,
};
use crate::plugin::logger::keys::state::{
    ACTION_EVENT_AMBIGUOUS, ACTION_EVENT_IGNORED, ACTION_EVENT_TRIGGERED, ACTION_SCOPE_MISMATCH,
    BLOCK_SCOPE_MISMATCH, FALLBACK_BLOCK as FALLBACK_BLOCK_STATE,
    FALLBACK_MODIFIER as FALLBACK_MODIFIER_STATE, FALLBACK_TRIGGER, FRAME_LOAD_FAILED,
    FRAME_LOAD_SUCCEED, FRAME_LOADING, MODIFIER_SCOPE_MISMATCH, TRIGGER_EXECUTED,
};
use crate::plugin::logger::keys::tag::{
    ACTION_SCOPE, BLOCK_SCOPE, FALLBACK_ACTION, FALLBACK_BLOCK, FALLBACK_MODIFIER, FRAME_STATE,
    HANDLE_ACTION, MODIFIER_SCOPE,
};
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
            ActionLogEvent::EventAmbiguous {
                event,
                block_key,
                count,
            } => self.dispatch(
                LoggerEventLevel::Debug,
                HANDLE_ACTION,
                format!(
                    "{count} actions are bound to '{event}' on block '{block_key}', only the first one runs"
                ),
                HashMap::from([
                    (STATE.to_string(), ACTION_EVENT_AMBIGUOUS.to_string()),
                    (EVENT_NAME.to_string(), event),
                    (BLOCK_KEY.to_string(), block_key),
                    (MATCH_COUNT.to_string(), count.to_string()),
                ]),
            ),
            ActionLogEvent::TriggerExecuted {
                name,
                key_type,
                event,
            } => self.dispatch(
                LoggerEventLevel::Debug,
                HANDLE_ACTION,
                format!("Trigger '{name}' executed"),
                HashMap::from([
                    (STATE.to_string(), TRIGGER_EXECUTED.to_string()),
                    (TRIGGER_NAME.to_string(), name),
                    (KEY_TYPE.to_string(), key_type),
                    (EVENT_NAME.to_string(), event),
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
            ActionLogEvent::ScopeMismatch {
                trigger_name,
                key_type,
                required,
                provided,
                dropped,
            } => self.dispatch(
                if dropped {
                    LoggerEventLevel::Error
                } else {
                    LoggerEventLevel::Debug
                },
                ACTION_SCOPE,
                if dropped {
                    format!(
                        "Trigger '{trigger_name}' requires '{required}' scope but sits under an event providing '{provided}', dropped"
                    )
                } else {
                    format!(
                        "Trigger '{trigger_name}' requires '{required}' scope but its parent event declares none, running without it"
                    )
                },
                HashMap::from([
                    (STATE.to_string(), ACTION_SCOPE_MISMATCH.to_string()),
                    (TRIGGER_NAME.to_string(), trigger_name),
                    (KEY_TYPE.to_string(), key_type),
                    (REQUIRED_SCOPE.to_string(), required),
                    (PROVIDED_SCOPE.to_string(), provided),
                ]),
            ),
        }
    }

    pub(crate) fn block(&self, event: BlockLogEvent) {
        if !self.enabled() {
            return;
        }
        match event {
            BlockLogEvent::BlockFallback {
                key_type,
                block_key,
            } => self.dispatch(
                LoggerEventLevel::Error,
                FALLBACK_BLOCK,
                format!("No block registered for '{key_type}'"),
                HashMap::from([
                    (STATE.to_string(), FALLBACK_BLOCK_STATE.to_string()),
                    (KEY_TYPE.to_string(), key_type),
                    (KEY.to_string(), block_key),
                ]),
            ),
            BlockLogEvent::ModifierFallback {
                key_type,
                block_key,
            } => self.dispatch(
                LoggerEventLevel::Error,
                FALLBACK_MODIFIER,
                format!("No modifier registered for '{key_type}'"),
                HashMap::from([
                    (STATE.to_string(), FALLBACK_MODIFIER_STATE.to_string()),
                    (KEY_TYPE.to_string(), key_type),
                    (BLOCK_KEY.to_string(), block_key),
                ]),
            ),
            BlockLogEvent::ScopeMismatch {
                block_key,
                key_type,
                required,
                provided,
                dropped,
            } => self.dispatch(
                if dropped {
                    LoggerEventLevel::Error
                } else {
                    LoggerEventLevel::Debug
                },
                BLOCK_SCOPE,
                if dropped {
                    format!(
                        "Block '{block_key}' requires '{required}' scope but sits in a slot providing '{provided}', dropped"
                    )
                } else {
                    format!(
                        "Block '{block_key}' requires '{required}' scope but its slot declares none, rendering without it"
                    )
                },
                HashMap::from([
                    (STATE.to_string(), BLOCK_SCOPE_MISMATCH.to_string()),
                    (BLOCK_KEY.to_string(), block_key),
                    (KEY_TYPE.to_string(), key_type),
                    (REQUIRED_SCOPE.to_string(), required),
                    (PROVIDED_SCOPE.to_string(), provided),
                ]),
            ),
            BlockLogEvent::ModifierScopeMismatch {
                block_key,
                key_type,
                required,
                provided,
                dropped,
            } => self.dispatch(
                if dropped {
                    LoggerEventLevel::Error
                } else {
                    LoggerEventLevel::Debug
                },
                MODIFIER_SCOPE,
                if dropped {
                    format!(
                        "Modifier '{key_type}' requires '{required}' scope but block '{block_key}' sits in a slot providing '{provided}', dropped"
                    )
                } else {
                    format!(
                        "Modifier '{key_type}' requires '{required}' scope but the slot of block '{block_key}' declares none, applying without it"
                    )
                },
                HashMap::from([
                    (STATE.to_string(), MODIFIER_SCOPE_MISMATCH.to_string()),
                    (BLOCK_KEY.to_string(), block_key),
                    (KEY_TYPE.to_string(), key_type),
                    (REQUIRED_SCOPE.to_string(), required),
                    (PROVIDED_SCOPE.to_string(), provided),
                ]),
            ),
        }
    }

    pub(crate) fn variable_changed(&self, key: &str) {
        if !self.enabled() {
            return;
        }
        self.dispatch(
            LoggerEventLevel::Debug,
            FRAME_STATE,
            format!("Variable '{key}' changed"),
            HashMap::new(),
        );
    }

    pub(crate) fn host_variable_write(&self, key: &str) {
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

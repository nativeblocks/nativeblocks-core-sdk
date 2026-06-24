use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use crate::common::environment::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::ErrorType;
use crate::frame::domain::model::{
    NativeActionModel, NativeActionTriggerModel, NativeActionTriggerThen, NativeBlockModel,
    NativeFrameModel, NativeVariableModel,
};
use crate::frame::domain::repository::FrameRepository;
use crate::frame::presenter::action_props::ActionProps;
use crate::frame::presenter::action_provider;
use crate::frame::presenter::block_props::BlockProps;
use crate::frame::presenter::global_parameter;

const VARIABLE_TYPE_STRING: &str = "STRING";

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FrameState {
    Initial { development_mode: bool },
    Frame { development_mode: bool },
    Error { development_mode: bool, message: String },
}

#[uniffi::export(with_foreign)]
pub trait FrameObserver: Send + Sync {
    fn on_state_changed(&self, state: FrameState);
    fn on_frame_loaded(&self, generation: u64);
    fn on_variable_changed(&self, variable: NativeVariableModel);
    fn on_block_changed(&self, block: NativeBlockModel);
}

struct State {
    frame_state: FrameState,
    variables: HashMap<String, NativeVariableModel>,
    blocks: HashMap<String, NativeBlockModel>,
    actions: HashMap<String, Vec<NativeActionModel>>,
    generation: u64,
    current_route: String,
}

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    me: Weak<FrameStateManager>,
    repository: Arc<dyn FrameRepository>,
    instance_name: String,
    development_mode: bool,
    sdk_config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    state: Mutex<State>,
    observer: Mutex<Option<Arc<dyn FrameObserver>>>,
}

impl FrameStateManager {
    pub(crate) fn new(
        repository: Arc<dyn FrameRepository>,
        instance_name: String,
        development_mode: bool,
        sdk_config: SdkConfig,
        logger: Arc<Mutex<NativeLoggerProvider>>,
    ) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            repository,
            instance_name,
            development_mode,
            sdk_config,
            logger,
            state: Mutex::new(State {
                frame_state: FrameState::Initial { development_mode },
                variables: HashMap::new(),
                blocks: HashMap::new(),
                actions: HashMap::new(),
                generation: 0,
                current_route: String::new(),
            }),
            observer: Mutex::new(None),
        });
    }

    fn arc(&self) -> Arc<FrameStateManager> {
        return self.me.upgrade().expect("FrameStateManager dropped");
    }

    fn observer(&self) -> Option<Arc<dyn FrameObserver>> {
        return self.observer.lock().unwrap().clone();
    }
}

// ---- host-facing API ------------------------------------------------------
#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub fn observe(&self, observer: Arc<dyn FrameObserver>) {
        *self.observer.lock().unwrap() = Some(observer.clone());
        observer.on_state_changed(self.frame_state());
    }

    pub fn release(&self) {
        *self.observer.lock().unwrap() = None;
    }

    pub async fn setup_frame(&self, route: String, args: HashMap<String, String>) {
        self.state.lock().unwrap().current_route = route.clone();
        self.load_from_cache(&route, &args);
        let globals = global_parameter::get_or_create(&self.instance_name).get();
        let _ = self.repository.sync(&route, &globals).await;
        self.load_from_cache(&route, &args);
    }

    pub fn frame_state(&self) -> FrameState {
        return self.state.lock().unwrap().frame_state.clone();
    }

    pub fn root_block(&self) -> Option<NativeBlockModel> {
        return self
            .state
            .lock()
            .unwrap()
            .blocks
            .values()
            .find(|block| block.parent_id.is_empty())
            .cloned();
    }

    pub fn block_props(&self, block: NativeBlockModel, list_item_index: i32) -> Arc<BlockProps> {
        return Arc::new(BlockProps::new(
            self.arc(),
            self.instance_name.clone(),
            list_item_index,
            block,
        ));
    }

    pub fn handle_variable(&self, variable: NativeVariableModel) {
        let key = variable.key.clone();
        let previous = {
            let mut state = self.state.lock().unwrap();
            let previous = state.variables.get(&key).map(|value| value.value.clone());
            state.variables.insert(key, variable.clone());
            previous
        };
        if self.development_mode {
            self.log_variable_change(&variable, previous.as_deref());
        }
        if let Some(observer) = self.observer() {
            observer.on_variable_changed(variable);
        }
    }

    pub async fn handle_action(
        &self,
        list_item_index: i32,
        action: Option<NativeActionModel>,
        performed_event_type: String,
    ) {
        let action = match action {
            Some(action) => action,
            None => return,
        };

        if action.event != performed_event_type {
            if self.development_mode {
                self.log_action_ignored(&action, &performed_event_type);
            }
            return;
        }
        self.log_action_triggered(&action);

        let root_triggers: Vec<NativeActionTriggerModel> = action
            .triggers
            .iter()
            .filter(|trigger| trigger.parent_id.is_empty())
            .cloned()
            .collect();
        for trigger in root_triggers {
            self.handle_trigger(list_item_index, &action, trigger).await;
        }
    }
}

// ---- internal engine ------------------------------------------------------
impl FrameStateManager {
    pub(super) fn find_variable(&self, key: &str) -> Option<NativeVariableModel> {
        return self.state.lock().unwrap().variables.get(key).cloned();
    }

    pub(super) fn find_block(&self, key: &str) -> Option<NativeBlockModel> {
        return self.state.lock().unwrap().blocks.get(key).cloned();
    }

    pub(super) fn find_action(&self, block_key: &str, event: &str) -> Option<NativeActionModel> {
        return self
            .state
            .lock()
            .unwrap()
            .actions
            .get(block_key)?
            .iter()
            .find(|action| action.event == event)
            .cloned();
    }

    pub(super) fn children(&self, parent_id: &str, slot: &str) -> Vec<NativeBlockModel> {
        let state = self.state.lock().unwrap();
        let mut children: Vec<NativeBlockModel> = state
            .blocks
            .values()
            .filter(|block| block.parent_id == parent_id && block.slot == slot)
            .cloned()
            .collect();
        children.sort_by_key(|block| block.position);
        return children;
    }

    pub(super) fn handle_block(&self, block: NativeBlockModel) {
        if block.key.is_empty() {
            return;
        }
        self.state
            .lock()
            .unwrap()
            .blocks
            .insert(block.key.clone(), block.clone());
        if self.development_mode {
            self.log_block_change(&block);
        }
        if let Some(observer) = self.observer() {
            observer.on_block_changed(block);
        }
    }

    pub(super) async fn handle_child_triggers(
        &self,
        list_item_index: i32,
        action: &NativeActionModel,
        parent: &NativeActionTriggerModel,
        then: NativeActionTriggerThen,
    ) {
        let children: Vec<NativeActionTriggerModel> = action
            .triggers
            .iter()
            .filter(|trigger| trigger.parent_id == parent.id && trigger.then == then)
            .cloned()
            .collect();
        for trigger in children {
            self.handle_trigger(list_item_index, action, trigger).await;
        }
    }

    async fn handle_trigger(
        &self,
        list_item_index: i32,
        action: &NativeActionModel,
        trigger: NativeActionTriggerModel,
    ) {
        let provider = action_provider::get_or_create(&self.instance_name);
        let handler = match provider.find(&trigger.key_type) {
            Some(handler) => handler,
            None => {
                self.log_fallback(action, &trigger);
                if let Some(fallback) = provider.fallback() {
                    fallback.handle(trigger.key_type.clone(), trigger.name.clone());
                }
                return;
            }
        };
        self.log_trigger_executed(action, &trigger);

        let props = Arc::new(ActionProps::new(
            self.arc(),
            self.instance_name.clone(),
            list_item_index,
            action.clone(),
            trigger,
        ));
        handler.handle(props).await;
    }

    fn load_from_cache(&self, route: &str, args: &HashMap<String, String>) {
        match self.repository.get(route) {
            Ok(frame) => self.apply_frame(frame, args),
            Err(error) => {
                let state = if error.error_type == ErrorType::Cache {
                    FrameState::Initial {
                        development_mode: self.development_mode,
                    }
                } else {
                    FrameState::Error {
                        development_mode: self.development_mode,
                        message: error.message,
                    }
                };
                self.set_state(state);
            }
        }
    }

    fn apply_frame(&self, frame: NativeFrameModel, args: &HashMap<String, String>) {
        let (generation, state) = {
            let mut state = self.state.lock().unwrap();
            state.variables = frame.variables.clone();
            state.blocks = frame.blocks.clone();
            state.actions = frame.actions.clone();
            let globals = global_parameter::get_or_create(&self.instance_name).get();
            for (key, value) in args.iter().chain(globals.iter()) {
                state.variables.insert(
                    key.clone(),
                    NativeVariableModel {
                        key: key.clone(),
                        value: value.clone(),
                        variable_type: VARIABLE_TYPE_STRING.to_string(),
                    },
                );
            }
            state.generation += 1;
            state.frame_state = FrameState::Frame {
                development_mode: self.development_mode,
            };
            (state.generation, state.frame_state.clone())
        };

        if self.development_mode {
            for variable in frame.variables.values() {
                self.log_variable_change(variable, None);
            }
            for block in frame.blocks.values() {
                self.log_block_change(block);
            }
        }

        if let Some(observer) = self.observer() {
            observer.on_frame_loaded(generation);
            observer.on_state_changed(state);
        }
    }

    fn set_state(&self, state: FrameState) {
        self.state.lock().unwrap().frame_state = state.clone();
        if let Some(observer) = self.observer() {
            observer.on_state_changed(state);
        }
    }
}

// ---- logging --------------------------------------------------------------
impl FrameStateManager {
    fn log(
        &self,
        level: LoggerEventLevel,
        event: &str,
        message: &str,
        mut params: HashMap<String, String>,
    ) {
        let route = self.state.lock().unwrap().current_route.clone();
        params.insert(keys::parameter::FRAME_ROUTE.to_string(), route);
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(&self.sdk_config, level, event, message, params);
        }
    }

    fn log_variable_change(&self, variable: &NativeVariableModel, previous: Option<&str>) {
        let changed = matches!(previous, Some(value) if value != variable.value);
        let message = if changed {
            format!(
                "Variable changed: {} changed from '{}' to '{}'",
                variable.key,
                previous.unwrap_or_default(),
                variable.value
            )
        } else {
            format!("Variable changed: {} set to '{}'", variable.key, variable.value)
        };
        let mut params = HashMap::from([
            (keys::parameter::STATE.to_string(), keys::state::VARIABLE_UPDATED.to_string()),
            (keys::parameter::KEY.to_string(), variable.key.clone()),
            (keys::parameter::NEW_VALUE.to_string(), variable.value.clone()),
            (keys::parameter::VARIABLE_TYPE.to_string(), variable.variable_type.clone()),
        ]);
        if changed {
            params.insert(
                keys::parameter::PREVIOUS_VALUE.to_string(),
                previous.unwrap_or_default().to_string(),
            );
        }
        let level = if self.development_mode {
            LoggerEventLevel::Debug
        } else {
            LoggerEventLevel::Info
        };
        self.log(level, keys::tag::VARIABLE_CHANGE, &message, params);
    }

    fn log_block_change(&self, block: &NativeBlockModel) {
        let level = if self.development_mode {
            LoggerEventLevel::Debug
        } else {
            LoggerEventLevel::Info
        };
        self.log(
            level,
            keys::tag::BLOCK_CHANGE,
            &format!("Block updated: {}[{}]", block.key, block.key_type),
            HashMap::from([
                (keys::parameter::STATE.to_string(), keys::state::BLOCK_UPDATED.to_string()),
                (keys::parameter::BLOCK_KEY.to_string(), block.key.clone()),
                (keys::parameter::KEY_TYPE.to_string(), block.key_type.clone()),
            ]),
        );
    }

    fn log_action_triggered(&self, action: &NativeActionModel) {
        self.log(
            LoggerEventLevel::Info,
            keys::tag::HANDLE_ACTION,
            &format!("Action event triggered: {} for {}", action.event, action.key),
            HashMap::from([
                (keys::parameter::STATE.to_string(), keys::state::ACTION_EVENT_TRIGGERED.to_string()),
                (keys::parameter::EVENT_NAME.to_string(), action.event.clone()),
                (keys::parameter::KEY.to_string(), action.key.clone()),
            ]),
        );
    }

    fn log_action_ignored(&self, action: &NativeActionModel, performed_event_type: &str) {
        self.log(
            LoggerEventLevel::Debug,
            keys::tag::HANDLE_ACTION,
            &format!(
                "Action event ignored: expected {}, received {}",
                action.event, performed_event_type
            ),
            HashMap::from([
                (keys::parameter::STATE.to_string(), keys::state::ACTION_EVENT_IGNORED.to_string()),
                (keys::parameter::EVENT_NAME.to_string(), performed_event_type.to_string()),
                (keys::parameter::KEY.to_string(), action.key.clone()),
                (keys::parameter::ACTION_NAME.to_string(), action.event.clone()),
            ]),
        );
    }

    fn log_trigger_executed(&self, action: &NativeActionModel, trigger: &NativeActionTriggerModel) {
        self.log(
            LoggerEventLevel::Info,
            keys::tag::HANDLE_ACTION,
            &format!("Executing trigger: {} [{}]", trigger.name, trigger.key_type),
            HashMap::from([
                (keys::parameter::STATE.to_string(), keys::state::TRIGGER_EXECUTED.to_string()),
                (
                    keys::parameter::ACTION_NAME.to_string(),
                    format!("{}[{}]", action.key, action.event),
                ),
                (keys::parameter::TRIGGER_NAME.to_string(), trigger.name.clone()),
                (keys::parameter::KEY_TYPE.to_string(), trigger.key_type.clone()),
            ]),
        );
    }

    fn log_fallback(&self, action: &NativeActionModel, trigger: &NativeActionTriggerModel) {
        self.log(
            LoggerEventLevel::Warning,
            keys::tag::FALLBACK_ACTION,
            &format!("Fallback action triggered: {} is not available", trigger.name),
            HashMap::from([
                (keys::parameter::STATE.to_string(), keys::state::FALLBACK_TRIGGER.to_string()),
                (keys::parameter::KEY_TYPE.to_string(), trigger.key_type.clone()),
                (keys::parameter::ACTION_NAME.to_string(), trigger.name.clone()),
                (keys::parameter::TRIGGER_NAME.to_string(), trigger.name.clone()),
                (keys::parameter::EVENT_NAME.to_string(), action.event.clone()),
            ]),
        );
    }
}

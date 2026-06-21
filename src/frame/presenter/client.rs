use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::sync::watch;

use crate::common::config::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::common::result::{ErrorType, NBResult};
use crate::frame::domain::action::{ActionContext, ActionResult, NativeActionHandler};
use crate::frame::data::repository::{FrameRepository, FrameStream};
use crate::frame::domain::model::{
    FrameSyncRequest, NativeActionModel, NativeActionTriggerModel, NativeBlockModel,
    NativeFrameModel, NativeFrameState, NativeVariableModel,
};
use crate::localization;

pub(crate) struct Client {
    repository: FrameRepository,
    localization: Arc<localization::Client>,
    config: SdkConfig,
    logger: Arc<Mutex<NativeLoggerProvider>>,
    development_mode: bool,
    instance_name: String,
    frame_state: watch::Sender<NativeFrameState>,
    blocks: watch::Sender<HashMap<String, NativeBlockModel>>,
    variables: watch::Sender<HashMap<String, NativeVariableModel>>,
    actions: watch::Sender<HashMap<String, Vec<NativeActionModel>>>,
    generation: watch::Sender<u32>,
    action_handlers: Mutex<HashMap<String, Arc<dyn NativeActionHandler>>>,
    current_route: Mutex<Option<String>>,
    route_arguments: Mutex<HashMap<String, String>>,
    frame_stream: Mutex<Option<FrameStream>>,
}

impl Client {
    pub(crate) fn new(
        repository: FrameRepository,
        localization: Arc<localization::Client>,
        config: SdkConfig,
        logger: Arc<Mutex<NativeLoggerProvider>>,
        development_mode: bool,
        instance_name: String,
    ) -> Self {
        let (frame_state, _) = watch::channel(NativeFrameState::initial(development_mode));
        let (blocks, _) = watch::channel(HashMap::new());
        let (variables, _) = watch::channel(HashMap::new());
        let (actions, _) = watch::channel(HashMap::new());
        let (generation, _) = watch::channel(0u32);

        return Self {
            repository,
            localization,
            config,
            logger,
            development_mode,
            instance_name,
            frame_state,
            blocks,
            variables,
            actions,
            generation,
            action_handlers: Mutex::new(HashMap::new()),
            current_route: Mutex::new(None),
            route_arguments: Mutex::new(HashMap::new()),
            frame_stream: Mutex::new(None),
        };
    }

    pub(crate) async fn setup_frame(&self, route: String, route_arguments: HashMap<String, String>) {
        *self.current_route.lock().unwrap() = Some(route.clone());
        *self.route_arguments.lock().unwrap() = route_arguments;

        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::FRAME_LOADING.to_string(),
        );
        self.log(
            LoggerEventLevel::Info,
            keys::tag::FRAME_STATE,
            format!("Frame loading: {route}"),
            params,
        );

        let stream = self.repository.get_frame(&route, self.development_mode);
        let value = stream.borrow().clone();
        *self.frame_stream.lock().unwrap() = Some(stream);
        self.apply_frame_result(value);
    }

    pub(crate) async fn sync_frame(&self, request: FrameSyncRequest) -> NBResult<()> {
        let route = request.route.clone();
        let result = self
            .repository
            .sync_frame(&request, self.development_mode)
            .await;

        match &result {
            Ok(()) => {
                let mut params = HashMap::new();
                params.insert(
                    keys::parameter::STATE.to_string(),
                    keys::state::FRAME_SYNC_SUCCEED.to_string(),
                );
                self.log(
                    LoggerEventLevel::Info,
                    keys::tag::FRAME_SYNC_STATE,
                    format!("Frame synced for {route}"),
                    params,
                );
                let latest = self
                    .frame_stream
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|rx| rx.borrow().clone());
                if let Some(value) = latest {
                    self.apply_frame_result(value);
                }
            }
            Err(error) => {
                let mut params = error.to_logger_parameters();
                params.insert(
                    keys::parameter::STATE.to_string(),
                    keys::state::FRAME_SYNC_FAILED.to_string(),
                );
                self.log(
                    LoggerEventLevel::Error,
                    keys::tag::FRAME_SYNC_STATE,
                    format!("Frame sync failed for {route}"),
                    params,
                );
            }
        }
        return result;
    }

    pub(crate) async fn sync_community_frame(
        &self,
        endpoint_frame: String,
        route: String,
    ) -> NBResult<()> {
        let result = self
            .repository
            .sync_community_frame(&endpoint_frame, &route)
            .await;
        if result.is_ok() {
            let latest = self
                .frame_stream
                .lock()
                .unwrap()
                .as_ref()
                .map(|rx| rx.borrow().clone());
            if let Some(value) = latest {
                self.apply_frame_result(value);
            }
        }
        return result;
    }

    pub(crate) fn handle_variable(&self, variable: NativeVariableModel, need_to_log: bool) {
        self.handle_variable_internal(variable, need_to_log);
    }

    pub(crate) async fn handle_action(
        &self,
        index: i32,
        action: Option<NativeActionModel>,
        performed_event_type: String,
    ) {
        if let Some(action) = action {
            self.run_event(index, action, performed_event_type).await;
        }
    }

    pub(crate) fn register_action_handler(
        &self,
        key_type: String,
        handler: Arc<dyn NativeActionHandler>,
    ) {
        self.action_handlers.lock().unwrap().insert(key_type, handler);
    }

    pub(crate) fn change_block(&self, block: NativeBlockModel) {
        self.block_hoist(block, true);
    }

    pub(crate) fn localize(&self, key: String) -> Option<String> {
        return self.localization.translate(key);
    }

    pub(crate) fn set_global_parameters(&self, parameters: HashMap<String, String>) {
        self.repository.set_global_parameters(parameters);
    }

    pub(crate) fn clear_all_frames(&self) -> NBResult<()> {
        return self.repository.clear_all_frames();
    }

    pub(crate) fn clear_frame(&self, route: String) -> NBResult<()> {
        return self.repository.clear_frame(&route);
    }

    pub(crate) fn native_frame_state(&self) -> NativeFrameState {
        return self.frame_state.borrow().clone();
    }

    pub(crate) fn blocks_state(&self) -> HashMap<String, NativeBlockModel> {
        return self.blocks.borrow().clone();
    }

    pub(crate) fn variables_state(&self) -> HashMap<String, NativeVariableModel> {
        return self.variables.borrow().clone();
    }

    pub(crate) fn action_state(&self) -> HashMap<String, Vec<NativeActionModel>> {
        return self.actions.borrow().clone();
    }

    pub(crate) fn frame_update_generation(&self) -> u32 {
        return *self.generation.borrow();
    }

    fn apply_frame_result(&self, value: NBResult<NativeFrameModel>) {
        match value {
            Ok(frame) => {
                self.generation.send_modify(|g| *g += 1);
                self.apply_frame_data(&frame);
                self.apply_external_arguments();
                let _ = self
                    .frame_state
                    .send_replace(NativeFrameState::success(self.development_mode));
            }
            Err(error) => {
                if error.error_type == ErrorType::Cache {
                    let _ = self
                        .frame_state
                        .send_replace(NativeFrameState::initial(self.development_mode));
                } else {
                    let _ = self
                        .frame_state
                        .send_replace(NativeFrameState::error(self.development_mode, error.message));
                }
            }
        }
    }

    fn apply_frame_data(&self, frame: &NativeFrameModel) {
        self.variables.send_modify(|m| m.clear());
        self.blocks.send_modify(|m| m.clear());
        self.actions.send_modify(|m| m.clear());

        if let Some(variables) = &frame.variables {
            for variable in variables.values() {
                self.handle_variable_internal(variable.clone(), self.development_mode);
            }
        }
        if let Some(actions) = &frame.actions {
            for (key, list) in actions {
                self.action_hoist(key, list.clone());
            }
        }
        if let Some(blocks) = &frame.blocks {
            for block in blocks.values() {
                self.block_hoist(block.clone(), self.development_mode);
            }
        }
    }

    fn apply_external_arguments(&self) {
        let mut external = self.route_arguments.lock().unwrap().clone();
        external.extend(self.repository.get_global_parameters());
        for (key, value) in external {
            self.handle_variable_internal(
                NativeVariableModel::new(key, value, "STRING"),
                self.development_mode,
            );
        }
    }

    fn handle_variable_internal(&self, variable: NativeVariableModel, need_to_log: bool) {
        let previous = self
            .variables
            .borrow()
            .get(&variable.key)
            .map(|v| v.value.clone());
        let key = variable.key.clone();
        let value = variable.value.clone();
        let value_type = variable.value_type.clone();
        self.variables.send_modify(|m| {
            m.insert(variable.key.clone(), variable);
        });

        if !need_to_log {
            return;
        }

        let is_changed = previous.as_ref().is_some_and(|p| *p != value);
        let message = if is_changed {
            format!(
                "Variable changed: {key} changed from '{}' to '{value}'",
                previous.clone().unwrap_or_default()
            )
        } else {
            format!("Variable changed: {key} set to '{value}'")
        };

        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::VARIABLE_UPDATED.to_string(),
        );
        params.insert(keys::parameter::KEY.to_string(), key);
        params.insert(keys::parameter::NEW_VALUE.to_string(), value);
        params.insert(keys::parameter::VARIABLE_TYPE.to_string(), value_type);
        if is_changed {
            params.insert(
                keys::parameter::PREVIOUS_VALUE.to_string(),
                previous.unwrap_or_default(),
            );
        }
        self.log(self.var_level(), keys::tag::VARIABLE_CHANGE, message, params);
    }

    fn block_hoist(&self, block: NativeBlockModel, need_to_log: bool) {
        if block.key.is_empty() {
            return;
        }
        let key = block.key.clone();
        let key_type = block.key_type.clone();
        self.blocks.send_modify(|m| {
            m.insert(block.key.clone(), block);
        });

        if need_to_log {
            let mut params = HashMap::new();
            params.insert(
                keys::parameter::STATE.to_string(),
                keys::state::BLOCK_UPDATED.to_string(),
            );
            params.insert(keys::parameter::BLOCK_KEY.to_string(), key.clone());
            params.insert(keys::parameter::KEY_TYPE.to_string(), key_type.clone());
            self.log(
                self.var_level(),
                keys::tag::BLOCK_CHANGE,
                format!("Block updated: {key}[{key_type}]"),
                params,
            );
        }
    }

    fn action_hoist(&self, key: &str, actions: Vec<NativeActionModel>) {
        self.actions.send_modify(|m| {
            m.insert(key.to_string(), actions);
        });
    }

    async fn run_event(&self, index: i32, action: NativeActionModel, performed_event_type: String) {
        if action.event != performed_event_type {
            if self.development_mode {
                let mut params = HashMap::new();
                params.insert(
                    keys::parameter::STATE.to_string(),
                    keys::state::ACTION_EVENT_IGNORED.to_string(),
                );
                params.insert(
                    keys::parameter::EVENT_NAME.to_string(),
                    performed_event_type.clone(),
                );
                params.insert(keys::parameter::KEY.to_string(), action.key.clone());
                params.insert(
                    keys::parameter::ACTION_NAME.to_string(),
                    action.event.clone(),
                );
                self.log(
                    LoggerEventLevel::Debug,
                    keys::tag::HANDLE_ACTION,
                    format!(
                        "Action event ignored: expected {}, received {performed_event_type}",
                        action.event
                    ),
                    params,
                );
            }
            return;
        }

        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::ACTION_EVENT_TRIGGERED.to_string(),
        );
        params.insert(keys::parameter::EVENT_NAME.to_string(), action.event.clone());
        params.insert(keys::parameter::KEY.to_string(), action.key.clone());
        self.log(
            LoggerEventLevel::Info,
            keys::tag::HANDLE_ACTION,
            format!("Action event triggered: {} for {}", action.event, action.key),
            params,
        );

        let roots: Vec<NativeActionTriggerModel> = action
            .triggers
            .iter()
            .filter(|t| t.parent_id.is_empty())
            .cloned()
            .collect();
        for trigger in &roots {
            self.run_trigger(&action, index, trigger).await;
        }
    }

    fn run_trigger<'a>(
        &'a self,
        action: &'a NativeActionModel,
        index: i32,
        trigger: &'a NativeActionTriggerModel,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        return Box::pin(async move {
            self.log_trigger(action, trigger);

            let outcome = if trigger.key_type == crate::frame::domain::action::SCRIPT_KEY_TYPE {
                self.run_script(trigger, index)
            } else if let Some(handler) = self.action_handler(&trigger.key_type) {
                let context = self.action_context(index, trigger);
                Some(handler.handle(context).await)
            } else {
                None
            };

            let result = match outcome {
                Some(result) => result,
                None => {
                    self.fallback_action(action, trigger);
                    return;
                }
            };

            self.apply_action_result(&result);

            let children: Vec<&NativeActionTriggerModel> = action
                .triggers
                .iter()
                .filter(|t| t.parent_id == trigger.id && t.then == result.then)
                .collect();
            for child in children {
                self.run_trigger(action, index, child).await;
            }
        });
    }

    fn action_handler(&self, key_type: &str) -> Option<Arc<dyn NativeActionHandler>> {
        return self.action_handlers.lock().unwrap().get(key_type).cloned();
    }

    fn action_context(&self, index: i32, trigger: &NativeActionTriggerModel) -> ActionContext {
        return ActionContext {
            instance_name: self.instance_name.clone(),
            list_item_index: index,
            key_type: trigger.key_type.clone(),
            trigger: trigger.clone(),
            variables: self.variables.borrow().clone(),
            blocks: self.blocks.borrow().clone(),
        };
    }

    fn apply_action_result(&self, result: &ActionResult) {
        for variable in &result.variable_changes {
            self.handle_variable_internal(variable.clone(), true);
        }
        for block in &result.block_changes {
            self.block_hoist(block.clone(), true);
        }
    }

    fn fallback_action(&self, action: &NativeActionModel, trigger: &NativeActionTriggerModel) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::FALLBACK_TRIGGER.to_string(),
        );
        params.insert(keys::parameter::KEY_TYPE.to_string(), trigger.key_type.clone());
        params.insert(keys::parameter::ACTION_NAME.to_string(), trigger.name.clone());
        params.insert(keys::parameter::TRIGGER_NAME.to_string(), trigger.name.clone());
        params.insert(keys::parameter::EVENT_NAME.to_string(), action.event.clone());
        self.log(
            LoggerEventLevel::Warning,
            keys::tag::FALLBACK_ACTION,
            format!("Fallback action triggered: {} is not available", trigger.name),
            params,
        );
    }

    fn log_trigger(&self, action: &NativeActionModel, trigger: &NativeActionTriggerModel) {
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::TRIGGER_EXECUTED.to_string(),
        );
        params.insert(
            keys::parameter::ACTION_NAME.to_string(),
            format!("{}[{}]", action.key, action.event),
        );
        params.insert(keys::parameter::TRIGGER_NAME.to_string(), trigger.name.clone());
        params.insert(keys::parameter::KEY_TYPE.to_string(), trigger.key_type.clone());
        params.insert(keys::parameter::THEN.to_string(), trigger.then.as_str().to_string());
        self.log(
            LoggerEventLevel::Info,
            keys::tag::HANDLE_ACTION,
            format!("Executing trigger: {} [{}]", trigger.name, trigger.key_type),
            params,
        );
    }

    #[cfg(feature = "script-quickjs")]
    fn run_script(&self, trigger: &NativeActionTriggerModel, index: i32) -> Option<ActionResult> {
        let bridge = crate::frame::data::script::ScriptBridge {
            variables: self.variables.clone(),
            blocks: self.blocks.clone(),
            logger: self.logger.clone(),
            config: self.config.clone(),
            development_mode: self.development_mode,
            route: self.current_route.lock().unwrap().clone().unwrap_or_default(),
        };
        return Some(crate::frame::data::script::evaluate(&bridge, trigger, index));
    }

    #[cfg(not(feature = "script-quickjs"))]
    fn run_script(&self, _trigger: &NativeActionTriggerModel, _index: i32) -> Option<ActionResult> {
        return None;
    }

    fn var_level(&self) -> LoggerEventLevel {
        return if self.development_mode {
            LoggerEventLevel::Debug
        } else {
            LoggerEventLevel::Info
        };
    }

    fn log(
        &self,
        level: LoggerEventLevel,
        event: &str,
        message: String,
        mut params: HashMap<String, String>,
    ) {
        let route = self.current_route.lock().unwrap().clone().unwrap_or_default();
        params.insert(keys::parameter::FRAME_ROUTE.to_string(), route);
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(&self.config, level, event, message, params);
        }
    }
}

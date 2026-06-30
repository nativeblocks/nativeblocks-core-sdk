use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use tokio::task::JoinHandle;

use crate::common::result::{ErrorType, NBResult};
use crate::frame::domain::model::{
    NativeActionModel, NativeBlockModel, NativeFrameModel, NativeVariableModel,
};
use crate::frame::domain::repository::FrameRepository;
use crate::global_parameter;

const VARIABLE_TYPE_STRING: &str = "STRING";

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FrameState {
    Loading {},
    Ready {},
    Error { message: String },
}

#[uniffi::export(with_foreign)]
pub trait FrameStateObserver: Send + Sync {
    fn on_state_changed(&self, state: FrameState);
    fn on_variables_changed(&self, variables: HashMap<String, NativeVariableModel>);
    fn on_blocks_changed(&self, blocks: HashMap<String, Vec<NativeBlockModel>>);
    fn on_actions_changed(&self, actions: HashMap<String, Vec<NativeActionModel>>);
}

struct State {
    frame_state: FrameState,
    variables: HashMap<String, NativeVariableModel>,
    blocks: HashMap<String, Vec<NativeBlockModel>>,
    actions: HashMap<String, Vec<NativeActionModel>>,
}

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    me: Weak<FrameStateManager>,
    repository: Arc<dyn FrameRepository>,
    instance_name: String,
    state: Mutex<State>,
    observer: Mutex<Option<Arc<dyn FrameStateObserver>>>,
    observe_task: Mutex<Option<JoinHandle<()>>>,
}

impl FrameStateManager {
    pub(crate) fn new(repository: Arc<dyn FrameRepository>, instance_name: String) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            repository,
            instance_name,
            state: Mutex::new(State {
                frame_state: FrameState::Loading {},
                variables: HashMap::new(),
                blocks: HashMap::new(),
                actions: HashMap::new(),
            }),
            observer: Mutex::new(None),
            observe_task: Mutex::new(None),
        });
    }

    fn observer(&self) -> Option<Arc<dyn FrameStateObserver>> {
        return self.observer.lock().unwrap().clone();
    }
}

// ---- host-facing API ------------------------------------------------------
#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub fn observe(&self, observer: Arc<dyn FrameStateObserver>) {
        *self.observer.lock().unwrap() = Some(observer.clone());
        let state = self.state.lock().unwrap();
        observer.on_state_changed(state.frame_state.clone());
        if state.frame_state == (FrameState::Ready {}) {
            observer.on_variables_changed(state.variables.clone());
            observer.on_blocks_changed(state.blocks.clone());
            observer.on_actions_changed(state.actions.clone());
        }
    }

    pub fn release(&self) {
        if let Some(task) = self.observe_task.lock().unwrap().take() {
            task.abort();
        }
        *self.observer.lock().unwrap() = None;
        let mut state = self.state.lock().unwrap();
        state.variables = HashMap::new();
        state.blocks = HashMap::new();
        state.actions = HashMap::new();
        state.frame_state = FrameState::Loading {};
    }

    pub async fn setup_frame(&self, route: String, args: HashMap<String, String>) {
        self.observe_cache(route.clone(), args);
        let globals = global_parameter::get_or_create(&self.instance_name).get();
        let _ = self.repository.sync(&route, &globals).await;
    }

    pub fn frame_state(&self) -> FrameState {
        return self.state.lock().unwrap().frame_state.clone();
    }
}

// ---- internal -------------------------------------------------------------
impl FrameStateManager {
    fn observe_cache(&self, route: String, args: HashMap<String, String>) {
        let mut receiver = self.repository.get(&route);
        let weak = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let result = receiver.borrow_and_update().clone();
                let Some(manager) = weak.upgrade() else { break };
                manager.apply_frame(result, &args);
                if receiver.changed().await.is_err() {
                    break;
                }
            }
        });
        if let Some(previous) = self.observe_task.lock().unwrap().replace(task) {
            previous.abort();
        }
    }

    fn apply_frame(&self, result: NBResult<NativeFrameModel>, args: &HashMap<String, String>) {
        let frame = match result {
            Ok(frame) => frame,
            Err(error) => {
                let state = if error.error_type == ErrorType::Cache {
                    FrameState::Loading {}
                } else {
                    FrameState::Error { message: error.message }
                };
                self.set_state(state);
                return;
            }
        };

        let (variables, blocks, actions) = {
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
            state.frame_state = FrameState::Ready {};
            (
                state.variables.clone(),
                state.blocks.clone(),
                state.actions.clone(),
            )
        };

        if let Some(observer) = self.observer() {
            observer.on_variables_changed(variables);
            observer.on_blocks_changed(blocks);
            observer.on_actions_changed(actions);
            observer.on_state_changed(FrameState::Ready {});
        }
    }

    fn set_state(&self, state: FrameState) {
        self.state.lock().unwrap().frame_state = state.clone();
        if let Some(observer) = self.observer() {
            observer.on_state_changed(state);
        }
    }
}

use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};

use tokio::task::JoinHandle;

use crate::feature::frame::domain::model::{
    NativeActionModel, NativeBlockModel, NativeFrameModel, NativeVariableModel,
};
use crate::feature::frame::domain::repository::{FrameRepository, FrameUpdate};
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::library::result::ErrorType;
use crate::plugin::global_parameter::GlobalParameterProvider;

const VARIABLE_TYPE_STRING: &str = "STRING";

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FrameState {
    Loading {},
    Ready {},
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FrameSnapshot {
    pub state: FrameState,
    pub blocks: HashMap<String, NativeBlockModel>,
    pub variables: HashMap<String, NativeVariableModel>,
    pub actions: HashMap<String, Vec<NativeActionModel>>,
}

#[uniffi::export(with_foreign)]
pub trait FrameStateObserver: Send + Sync {
    fn on_frame_changed(&self, snapshot: FrameSnapshot);
}

struct State {
    frame_state: FrameState,
    frame: Option<Arc<NativeFrameModel>>,
    variables: HashMap<String, NativeVariableModel>,
}

impl State {
    fn fresh() -> Self {
        return Self {
            frame_state: FrameState::Loading {},
            frame: None,
            variables: HashMap::new(),
        };
    }

    fn blocks(&self) -> HashMap<String, NativeBlockModel> {
        return self
            .frame
            .as_ref()
            .map(|frame| frame.blocks.clone())
            .unwrap_or_default();
    }

    fn actions(&self) -> HashMap<String, Vec<NativeActionModel>> {
        return self
            .frame
            .as_ref()
            .map(|frame| frame.actions.clone())
            .unwrap_or_default();
    }

    fn snapshot(&self) -> FrameSnapshot {
        return FrameSnapshot {
            state: self.frame_state.clone(),
            blocks: self.blocks(),
            variables: self.variables.clone(),
            actions: self.actions(),
        };
    }

    fn is_fresh(&self) -> bool {
        return self.frame.is_none() && matches!(self.frame_state, FrameState::Loading {});
    }
}

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    me: Weak<FrameStateManager>,
    repository: Arc<dyn FrameRepository>,
    globals: Arc<GlobalParameterProvider>,
    state: Mutex<State>,
    observer: Mutex<Option<Arc<dyn FrameStateObserver>>>,
    observe_task: Mutex<Option<JoinHandle<()>>>,
    logger: FrameLogger,
}

impl FrameStateManager {
    pub(crate) fn new(
        repository: Arc<dyn FrameRepository>,
        globals: Arc<GlobalParameterProvider>,
        logger: FrameLogger,
    ) -> Arc<Self> {
        return Arc::new_cyclic(|me| Self {
            me: me.clone(),
            repository,
            globals,
            state: Mutex::new(State::fresh()),
            observer: Mutex::new(None),
            observe_task: Mutex::new(None),
            logger,
        });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub async fn setup_frame(&self, route: String, args: HashMap<String, String>) {
        self.logger.set_route(&route);
        *self.state.lock().unwrap() = State::fresh();
        self.observe_cache(route.clone(), args);
        let globals = self.globals.get();
        let _ = self.repository.load(&route, &globals).await;
    }

    pub fn observe(&self, observer: Arc<dyn FrameStateObserver>) {
        *self.observer.lock().unwrap() = Some(observer.clone());
        let snapshot = {
            let state = self.state.lock().unwrap();
            if state.is_fresh() {
                None
            } else {
                Some(state.snapshot())
            }
        };
        if let Some(snapshot) = snapshot {
            observer.on_frame_changed(snapshot);
        }
    }

    pub fn release(&self) {
        if let Some(task) = self.observe_task.lock().unwrap().take() {
            task.abort();
        }
        *self.observer.lock().unwrap() = None;
        *self.state.lock().unwrap() = State::fresh();
    }

    pub fn frame_state(&self) -> FrameState {
        return self.state.lock().unwrap().frame_state.clone();
    }

    pub fn root_block_id(&self) -> Option<String> {
        return self
            .state
            .lock()
            .unwrap()
            .frame
            .as_ref()
            .and_then(|frame| frame.root_id.clone());
    }

    pub fn blocks(&self) -> HashMap<String, NativeBlockModel> {
        return self.state.lock().unwrap().blocks();
    }

    pub fn variables(&self) -> HashMap<String, NativeVariableModel> {
        return self.state.lock().unwrap().variables.clone();
    }

    pub fn actions(&self) -> HashMap<String, Vec<NativeActionModel>> {
        return self.state.lock().unwrap().actions();
    }
}

impl FrameStateManager {
    fn observe_cache(&self, route: String, args: HashMap<String, String>) {
        let mut frame_receiver = self.repository.subscribe(&route);
        let weak_manager = self.me.clone();
        let task = tokio::spawn(async move {
            loop {
                let frame_result = frame_receiver.borrow_and_update().clone();
                let Some(manager) = weak_manager.upgrade() else {
                    break;
                };
                manager.apply_frame(frame_result, &args);
                if frame_receiver.changed().await.is_err() {
                    break;
                }
            }
        });
        if let Some(previous) = self.observe_task.lock().unwrap().replace(task) {
            previous.abort();
        }
    }

    fn apply_frame(&self, result: FrameUpdate, args: &HashMap<String, String>) {
        let frame = match result {
            Ok(frame) => frame,
            Err(error) => {
                let state = if error.error_type == ErrorType::Cache {
                    FrameState::Loading {}
                } else {
                    FrameState::Error {
                        message: error.message,
                    }
                };
                self.set_state(state);
                return;
            }
        };

        let variables = merge_variables(frame.variables.clone(), args, &self.globals.get());

        let snapshot = FrameSnapshot {
            state: FrameState::Ready {},
            blocks: frame.blocks.clone(),
            variables: variables.clone(),
            actions: frame.actions.clone(),
        };

        {
            let mut state = self.state.lock().unwrap();
            state.frame = Some(frame);
            state.variables = variables;
            state.frame_state = FrameState::Ready {};
        }

        self.logger.frame_state_changed(&FrameState::Ready {});
        if let Some(observer) = self.observer() {
            observer.on_frame_changed(snapshot);
        }
    }

    fn set_state(&self, state: FrameState) {
        let snapshot = {
            let mut guard = self.state.lock().unwrap();
            guard.frame_state = state.clone();
            guard.snapshot()
        };
        self.logger.frame_state_changed(&state);
        if let Some(observer) = self.observer() {
            observer.on_frame_changed(snapshot);
        }
    }

    fn observer(&self) -> Option<Arc<dyn FrameStateObserver>> {
        return self.observer.lock().unwrap().clone();
    }
}

impl Drop for FrameStateManager {
    fn drop(&mut self) {
        if let Ok(mut task) = self.observe_task.lock() {
            if let Some(task) = task.take() {
                task.abort();
            }
        }
    }
}

fn merge_variables(
    mut variables: HashMap<String, NativeVariableModel>,
    args: &HashMap<String, String>,
    globals: &HashMap<String, String>,
) -> HashMap<String, NativeVariableModel> {
    for (key, value) in args.iter().chain(globals.iter()) {
        variables.insert(
            key.clone(),
            NativeVariableModel {
                key: key.clone(),
                value: value.clone(),
                variable_type: VARIABLE_TYPE_STRING.to_string(),
            },
        );
    }
    return variables;
}

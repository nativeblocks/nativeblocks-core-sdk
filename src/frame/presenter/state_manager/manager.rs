use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, Weak};
use tokio::task::JoinHandle;

use crate::frame::domain::model::{NativeActionModel, NativeBlockModel, NativeVariableModel};
use crate::frame::domain::repository::FrameRepository;
use crate::frame::presenter::action_context::HostActionDispatcher;
use crate::frame::presenter::block_context::BlockObserver;
use crate::frame::presenter::logging::FrameLogger;
use crate::global_parameter::GlobalParameterProvider;
use crate::localization::LocalizationStateManager;

use super::{FrameState, FrameStateObserver};

pub(super) struct State {
    pub(super) frame_state: FrameState,
    pub(super) variables: HashMap<String, NativeVariableModel>,
    pub(super) blocks: HashMap<String, Vec<NativeBlockModel>>,
    pub(super) root_id: Option<String>,
    pub(super) actions: HashMap<String, Vec<NativeActionModel>>,
    pub(super) variable_dependent_block_ids: HashMap<String, HashSet<String>>,
    pub(super) block_observers: HashMap<String, Vec<(i32, Arc<dyn BlockObserver>)>>,
}

impl State {
    pub(super) fn fresh() -> Self {
        return Self {
            frame_state: FrameState::Loading {},
            variables: HashMap::new(),
            blocks: HashMap::new(),
            root_id: None,
            actions: HashMap::new(),
            variable_dependent_block_ids: HashMap::new(),
            block_observers: HashMap::new(),
        };
    }
}

#[derive(uniffi::Object)]
pub struct FrameStateManager {
    pub(super) me: Weak<FrameStateManager>,
    pub(super) repository: Arc<dyn FrameRepository>,
    pub(super) globals: Arc<GlobalParameterProvider>,
    pub(super) state: Mutex<State>,
    pub(super) observer: Mutex<Option<Arc<dyn FrameStateObserver>>>,
    pub(super) observe_task: Mutex<Option<JoinHandle<()>>>,
    pub(super) action_dispatcher: Mutex<Option<Arc<dyn HostActionDispatcher>>>,
    pub(super) localization: Mutex<Option<Arc<LocalizationStateManager>>>,
    pub(super) logger: FrameLogger,
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
            action_dispatcher: Mutex::new(None),
            localization: Mutex::new(None),
            logger,
        });
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

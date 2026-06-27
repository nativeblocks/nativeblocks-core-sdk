use std::collections::HashMap;
use std::sync::{Arc, Mutex, Weak};
use std::future::Future;
use std::pin::Pin;
use tokio::task::JoinHandle;

use crate::common::result::{ErrorType, NBResult};
use crate::frame::domain::model::{
    NativeActionModel, NativeBlockModel, NativeFrameModel, NativeVariableModel,
};
use crate::frame::domain::repository::FrameRepository;
use crate::frame::presenter::action_props::{ChangeBlock, FindBlock};
use crate::frame::presenter::action_provider::ActionFinder;
use crate::frame::presenter::action_tree;
use crate::frame::presenter::block_props::{
    FindVariable, HandleAction, Localize, VariableChange,
};
use crate::frame::presenter::block_provider::BlockFinder;
use crate::frame::presenter::block_tree::{self, FindActionByKey, FindSubBlocks};
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
    fn on_invalidate(&self);
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
    batch: Mutex<Option<bool>>,
}

impl FrameStateManager {
    pub(crate) fn new(
        repository: Arc<dyn FrameRepository>,
        instance_name: String,
    ) -> Arc<Self> {
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
            batch: Mutex::new(None),
        });
    }

    fn arc(&self) -> Arc<FrameStateManager> {
        return self.me.upgrade().expect("FrameStateManager dropped");
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
        observer.on_state_changed(self.frame_state());
    }

    pub fn release(&self) {
        if let Some(task) = self.observe_task.lock().unwrap().take() {
            task.abort();
        }
        *self.observer.lock().unwrap() = None;
        *self.batch.lock().unwrap() = None;
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

    pub fn render(&self, block_finder: Arc<dyn BlockFinder>, action_finder: Arc<dyn ActionFinder>) {
        let blocks = Arc::new(self.state.lock().unwrap().blocks.clone());
        let find_sub_blocks: FindSubBlocks = Arc::new(move |parent_id: &str| {
            return blocks.get(parent_id).cloned().unwrap_or_default();
        });

        let find_variable_manager = self.arc();
        let find_variable: FindVariable = Arc::new(move |key: String| {
            return find_variable_manager.find_variable(&key);
        });

        let variable_change_manager = self.arc();
        let variable_change: VariableChange = Arc::new(move |variable: NativeVariableModel| {
            return variable_change_manager.change_variable(variable)
        });

        let localize: Localize = Arc::new(|_key: String| {
            return None;
        });

        let find_block_manager = self.arc();
        let find_block: FindBlock = Arc::new(move |key: String| {
            return find_block_manager.find_block(&key);
        });

        let change_block_manager = self.arc();
        let change_block: ChangeBlock = Arc::new(move |block: NativeBlockModel| {
            return change_block_manager.change_block(block);
        });

        let find_action_manager = self.arc();
        let find_action: FindActionByKey = Arc::new(move |block_key: String, event: String| {
            find_action_manager.find_action(&block_key, &event)
        });

        let handle_action = self.handle_action_callback(
            find_variable.clone(),
            variable_change.clone(),
            find_block.clone(),
            change_block,
            action_finder,
        );

        block_tree::render(
            self.instance_name.clone(),
            find_sub_blocks,
            find_variable,
            variable_change,
            localize,
            find_action,
            find_block,
            handle_action,
            block_finder,
        );
    }
}

// ---- internal engine ------------------------------------------------------
impl FrameStateManager {
    fn find_variable(&self, key: &str) -> Option<NativeVariableModel> {
        return self.state.lock().unwrap().variables.get(key).cloned();
    }

    fn change_variable(&self, variable: NativeVariableModel) {
        let key = variable.key.clone();
        let changed = {
            let mut state = self.state.lock().unwrap();
            let same = state.variables.get(&key).map(|existing| existing.value.as_str()).unwrap_or_default() == variable.value.as_str();
            if same {
                false
            } else {
                state.variables.insert(key, variable);
                true
            }
        };
        if changed {
            self.notify();
        }
    }

    fn notify(&self) {
        let mut batch = self.batch.lock().unwrap();
        match batch.as_mut() {
            Some(dirty) => {
                *dirty = true;
            }
            None => {
                drop(batch);
                if let Some(observer) = self.observer() {
                    observer.on_invalidate();
                }
            }
        }
    }

    fn begin_batch(&self) {
        *self.batch.lock().unwrap() = Some(false);
    }

    fn end_batch(&self) {
        let dirty = self.batch.lock().unwrap().take().unwrap_or(false);
        if dirty {
            if let Some(observer) = self.observer() {
                observer.on_invalidate();
            }
        }
    }

    fn find_block(&self, key: &str) -> Option<NativeBlockModel> {
        let state = self.state.lock().unwrap();
        for children in state.blocks.values() {
            if let Some(block) = children.iter().find(|block| block.key == key) {
                return Some(block.clone());
            }
        }
        return None;
    }

    fn change_block(&self, block: NativeBlockModel) {
        let key = block.key.clone();
        {
            let mut state = self.state.lock().unwrap();
            if let Some(children) = state.blocks.get_mut(&block.parent_id) {
                if let Some(existing) = children.iter_mut().find(|existing| existing.key == key) {
                    *existing = block;
                }
            }
        }
        self.notify();
    }

    fn find_action(&self, block_key: &str, event: &str) -> Option<NativeActionModel> {
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

    fn handle_action_callback(
        &self,
        find_variable: FindVariable,
        variable_change: VariableChange,
        find_block: FindBlock,
        change_block: ChangeBlock,
        action_finder: Arc<dyn ActionFinder>,
    ) -> HandleAction {
        let instance_name = self.instance_name.clone();
        let me = self.arc();
        return Arc::new(
            move |list_item_index: i32, action: Option<NativeActionModel>, event_type: String| {
                let action = match action {
                    Some(action) => action,
                    None => return Box::pin(async {}) as Pin<Box<dyn Future<Output = ()> + Send>>,
                };
                let instance_name = instance_name.clone();
                let me = me.clone();
                let find_variable = find_variable.clone();
                let variable_change = variable_change.clone();
                let find_block = find_block.clone();
                let change_block = change_block.clone();
                let action_finder = action_finder.clone();
                return Box::pin(async move {
                    me.begin_batch();
                    action_tree::execute_action(
                        instance_name,
                        list_item_index,
                        action,
                        event_type,
                        find_variable,
                        variable_change,
                        find_block,
                        change_block,
                        action_finder,
                    )
                    .await;
                    me.end_batch();
                });
            },
        );
    }

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

        {
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
        }

        if let Some(observer) = self.observer() {
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

use std::collections::HashMap;
use std::sync::Arc;

use crate::frame::domain::model::{NativeBlockModel, NativeVariableModel};
use crate::frame::presenter::action_context::HostActionDispatcher;
use crate::frame::presenter::block_context::BlockContext;
use crate::localization::LocalizationStateManager;

use super::manager::State;
use super::FrameStateManager;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum FrameState {
    Loading {},
    Ready {},
    Error { message: String },
}

#[uniffi::export(with_foreign)]
pub trait FrameStateObserver: Send + Sync {
    fn on_state_changed(&self, state: FrameState);
}

#[uniffi::export(async_runtime = "tokio")]
impl FrameStateManager {
    pub fn observe(&self, observer: Arc<dyn FrameStateObserver>) {
        *self.observer.lock().unwrap() = Some(observer.clone());
        let state = self.state.lock().unwrap();
        observer.on_state_changed(state.frame_state.clone());
    }

    pub fn release(&self) {
        if let Some(task) = self.observe_task.lock().unwrap().take() {
            task.abort();
        }
        *self.observer.lock().unwrap() = None;
        *self.action_dispatcher.lock().unwrap() = None;
        *self.localization.lock().unwrap() = None;
        *self.state.lock().unwrap() = State::fresh();
    }

    pub async fn setup_frame(&self, route: String, args: HashMap<String, String>) {
        self.logger.set_route(&route);
        {
            let mut state = self.state.lock().unwrap();
            let block_observers = std::mem::take(&mut state.block_observers);
            *state = State::fresh();
            state.block_observers = block_observers;
        }
        self.observe_cache(route.clone(), args);
        let globals = self.globals.get();
        let _ = self.repository.sync(&route, &globals).await;
    }

    pub fn frame_state(&self) -> FrameState {
        return self.state.lock().unwrap().frame_state.clone();
    }

    pub fn root_block(self: Arc<Self>) -> Option<Arc<BlockContext>> {
        let id = self.state.lock().unwrap().root_id.clone()?;
        return Some(BlockContext::new(self.clone(), id, -1));
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return self.variable(&key);
    }

    pub fn variable_change(&self, variable: NativeVariableModel) {
        self.mutate_variable(variable);
    }

    pub fn find_block(&self, key: String) -> Option<NativeBlockModel> {
        return self.block_by_key(&key);
    }

    pub fn change_block(&self, block: NativeBlockModel) {
        self.mutate_block(block);
    }

    pub fn set_action_dispatcher(&self, dispatcher: Arc<dyn HostActionDispatcher>) {
        *self.action_dispatcher.lock().unwrap() = Some(dispatcher);
    }

    pub fn set_localization(&self, localization: Arc<LocalizationStateManager>) {
        *self.localization.lock().unwrap() = Some(localization);
    }

    pub fn invalidate_all(&self) {
        let subscribed: Vec<String> = self.state.lock().unwrap().block_observers.keys().cloned().collect();
        self.notify_block_observers(&subscribed);
    }
}

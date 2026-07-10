use std::sync::Arc;

use crate::frame::domain::model::{NativeActionModel, NativeBlockModel, NativeVariableModel};
use crate::frame::presenter::state_manager::FrameStateManager;

#[uniffi::export(with_foreign)]
pub trait BlockObserver: Send + Sync {
    fn on_invalidate(&self);
}

#[derive(uniffi::Object)]
pub struct BlockContext {
    manager: Arc<FrameStateManager>,
    id: String,
    index: i32,
}

impl BlockContext {
    pub(crate) fn new(
        manager: Arc<FrameStateManager>,
        id: String,
        index: i32,
    ) -> Arc<Self> {
        return Arc::new(Self {
            manager,
            id,
            index,
        });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl BlockContext {
    pub fn identity(&self) -> String {
        return self.id.clone();
    }

    pub fn index(&self) -> i32 {
        return self.index;
    }

    pub fn block(&self) -> Option<NativeBlockModel> {
        return self.manager.block_by_id(&self.id);
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return self.manager.variable(&key);
    }

    pub fn find_action(&self, event: String) -> Option<NativeActionModel> {
        return self.manager.action_of(&self.id, &event);
    }

    pub fn translate(&self, key: String) -> Option<String> {
        return self.manager.translate(&key);
    }

    pub fn children_of_slot(&self, slot: String) -> Vec<Arc<BlockContext>> {
        let child_ids = self.manager.children(&self.id, &slot);
        return child_ids
            .into_iter()
            .map(|child_id| BlockContext::new(self.manager.clone(), child_id, self.index))
            .collect();
    }

    pub fn variable_change(&self, variable: NativeVariableModel) {
        self.manager.mutate_variable(variable);
    }

    pub fn subscribe(&self, observer: Arc<dyn BlockObserver>) {
        self.manager.subscribe_block(&self.id, self.index, observer);
    }

    pub fn unsubscribe(&self) {
        self.manager.unsubscribe_block(&self.id, self.index);
    }

    pub async fn handle_action(&self, index: i32, event: String) {
        self.manager.handle_event(index, &self.id, &event);
    }
}

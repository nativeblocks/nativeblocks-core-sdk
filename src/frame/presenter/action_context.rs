use std::collections::HashMap;
use std::sync::Arc;

use crate::frame::domain::model::{
    NativeActionTriggerModel, NativeActionTriggerThen, NativeBlockModel, NativeVariableModel,
};
use crate::frame::presenter::state_manager::FrameStateManager;

#[uniffi::export(with_foreign)]
pub trait HostActionDispatcher: Send + Sync {
    fn dispatch(&self, key_type: String, props: Arc<ActionContext>);
}

#[derive(uniffi::Object)]
pub struct ActionContext {
    manager: Arc<FrameStateManager>,
    index: i32,
    trigger: NativeActionTriggerModel,
    by_parent: Arc<HashMap<String, Vec<NativeActionTriggerModel>>>,
    action_label: Arc<str>,
}

impl ActionContext {
    pub(crate) fn new(
        manager: Arc<FrameStateManager>,
        index: i32,
        trigger: NativeActionTriggerModel,
        by_parent: Arc<HashMap<String, Vec<NativeActionTriggerModel>>>,
        action_label: Arc<str>,
    ) -> Arc<Self> {
        return Arc::new(Self {
            manager,
            index,
            trigger,
            by_parent,
            action_label,
        });
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ActionContext {
    pub fn list_item_index(&self) -> i32 {
        return self.index;
    }

    pub fn trigger(&self) -> NativeActionTriggerModel {
        return self.trigger.clone();
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return self.manager.variable(&key);
    }

    pub fn find_block(&self, key: String) -> Option<NativeBlockModel> {
        return self.manager.block_by_key(&key);
    }

    pub fn variable_change(&self, variable: NativeVariableModel) {
        self.manager.mutate_variable(variable);
    }

    pub fn change_block(&self, block: NativeBlockModel) {
        self.manager.mutate_block(block);
    }

    pub async fn success(&self) {
        self.manager.advance(
            self.index,
            &self.trigger,
            NativeActionTriggerThen::Success,
            &self.by_parent,
            &self.action_label,
        );
    }

    pub async fn failure(&self) {
        self.manager.advance(
            self.index,
            &self.trigger,
            NativeActionTriggerThen::Failure,
            &self.by_parent,
            &self.action_label,
        );
    }

    pub async fn next(&self) {
        self.manager.advance(
            self.index,
            &self.trigger,
            NativeActionTriggerThen::Next,
            &self.by_parent,
            &self.action_label,
        );
    }
}

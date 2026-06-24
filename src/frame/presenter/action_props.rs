use std::sync::Arc;

use crate::frame::domain::model::{
    NativeActionModel, NativeActionTriggerModel, NativeActionTriggerThen, NativeBlockModel,
    NativeVariableModel,
};
use crate::frame::presenter::state_manager::FrameStateManager;

#[derive(uniffi::Object)]
pub struct ActionProps {
    manager: Arc<FrameStateManager>,
    instance_name: String,
    list_item_index: i32,
    action: NativeActionModel,
    trigger: NativeActionTriggerModel,
}

impl ActionProps {
    pub(super) fn new(
        manager: Arc<FrameStateManager>,
        instance_name: String,
        list_item_index: i32,
        action: NativeActionModel,
        trigger: NativeActionTriggerModel,
    ) -> Self {
        return Self {
            manager,
            instance_name,
            list_item_index,
            action,
            trigger,
        };
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl ActionProps {
    pub fn instance_name(&self) -> String {
        return self.instance_name.clone();
    }

    pub fn list_item_index(&self) -> i32 {
        return self.list_item_index;
    }

    pub fn trigger(&self) -> NativeActionTriggerModel {
        return self.trigger.clone();
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return self.manager.find_variable(&key);
    }

    pub fn change_variable(&self, variable: NativeVariableModel) {
        self.manager.handle_variable(variable);
    }

    pub fn find_block(&self, key: String) -> Option<NativeBlockModel> {
        return self.manager.find_block(&key);
    }

    pub fn change_block(&self, block: NativeBlockModel) {
        self.manager.handle_block(block);
    }

    pub async fn handle_next_triggers(&self) {
        self.manager
            .handle_child_triggers(
                self.list_item_index,
                &self.action,
                &self.trigger,
                NativeActionTriggerThen::Next,
            )
            .await;
    }

    pub async fn handle_success_triggers(&self) {
        self.manager
            .handle_child_triggers(
                self.list_item_index,
                &self.action,
                &self.trigger,
                NativeActionTriggerThen::Success,
            )
            .await;
    }

    pub async fn handle_failure_triggers(&self) {
        self.manager
            .handle_child_triggers(
                self.list_item_index,
                &self.action,
                &self.trigger,
                NativeActionTriggerThen::Failure,
            )
            .await;
    }
}

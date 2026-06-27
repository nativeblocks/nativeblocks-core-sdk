use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::frame::domain::model::{
    NativeActionTriggerModel, NativeBlockModel, NativeVariableModel,
};

pub(super) type FindVariable = Arc<dyn Fn(String) -> Option<NativeVariableModel> + Send + Sync>;
pub(super) type VariableChange = Arc<dyn Fn(NativeVariableModel) + Send + Sync>;
pub(super) type FindBlock = Arc<dyn Fn(String) -> Option<NativeBlockModel> + Send + Sync>;
pub(super) type ChangeBlock = Arc<dyn Fn(NativeBlockModel) + Send + Sync>;
pub(super) type HandleTrigger = Arc<dyn Fn(NativeActionTriggerModel) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(uniffi::Object)]
pub struct ActionProps {
    instance_name: String,
    list_item_index: i32,
    trigger: NativeActionTriggerModel,
    find_variable: FindVariable,
    variable_change: VariableChange,
    find_block: FindBlock,
    change_block: ChangeBlock,
    on_handle_next_trigger: HandleTrigger,
    on_handle_success_next_trigger: HandleTrigger,
    on_handle_failure_next_trigger: HandleTrigger,
}

impl ActionProps {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        instance_name: String,
        list_item_index: i32,
        trigger: NativeActionTriggerModel,
        find_variable: FindVariable,
        variable_change: VariableChange,
        find_block: FindBlock,
        change_block: ChangeBlock,
        on_handle_next_trigger: HandleTrigger,
        on_handle_success_next_trigger: HandleTrigger,
        on_handle_failure_next_trigger: HandleTrigger,
    ) -> Self {
        return Self {
            instance_name,
            list_item_index,
            trigger,
            find_variable,
            variable_change,
            find_block,
            change_block,
            on_handle_next_trigger,
            on_handle_success_next_trigger,
            on_handle_failure_next_trigger,
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
        return (self.find_variable)(key);
    }

    pub fn variable_change(&self, variable: NativeVariableModel) {
        (self.variable_change)(variable);
    }

    pub fn find_block(&self, key: String) -> Option<NativeBlockModel> {
        return (self.find_block)(key);
    }

    pub fn change_block(&self, block: NativeBlockModel) {
        (self.change_block)(block);
    }

    pub async fn handle_next_trigger(&self, trigger: NativeActionTriggerModel) {
        (self.on_handle_next_trigger)(trigger).await;
    }

    pub async fn handle_success_next_trigger(&self, trigger: NativeActionTriggerModel) {
        (self.on_handle_success_next_trigger)(trigger).await;
    }

    pub async fn handle_failure_next_trigger(&self, trigger: NativeActionTriggerModel) {
        (self.on_handle_failure_next_trigger)(trigger).await;
    }
}

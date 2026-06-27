use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::frame::domain::model::{NativeActionModel, NativeBlockModel, NativeVariableModel};

pub(super) type FindVariable = Arc<dyn Fn(String) -> Option<NativeVariableModel> + Send + Sync>;
pub(super) type VariableChange = Arc<dyn Fn(NativeVariableModel) + Send + Sync>;
pub(super) type Localize = Arc<dyn Fn(String) -> Option<String> + Send + Sync>;
pub(super) type SubBlock = Arc<dyn Fn(String, i32) + Send + Sync>;
pub(super) type FindAction = Arc<dyn Fn(String) -> Option<NativeActionModel> + Send + Sync>;
pub(super) type CurrentBlock = Arc<dyn Fn() -> Option<NativeBlockModel> + Send + Sync>;
pub(super) type HandleAction = Arc<dyn Fn(i32, Option<NativeActionModel>, String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(uniffi::Object)]
pub struct BlockProps {
    instance_name: String,
    list_item_index: i32,
    current_block: CurrentBlock,
    find_variable: FindVariable,
    variable_change: VariableChange,
    localize: Localize,
    on_sub_block: SubBlock,
    find_action: FindAction,
    handle_action: HandleAction,
}

impl BlockProps {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        instance_name: String,
        list_item_index: i32,
        current_block: CurrentBlock,
        find_variable: FindVariable,
        variable_change: VariableChange,
        localize: Localize,
        on_sub_block: SubBlock,
        find_action: FindAction,
        handle_action: HandleAction,
    ) -> Self {
        return Self {
            instance_name,
            list_item_index,
            current_block,
            find_variable,
            variable_change,
            localize,
            on_sub_block,
            find_action,
            handle_action,
        };
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl BlockProps {
    pub fn instance_name(&self) -> String {
        return self.instance_name.clone();
    }

    pub fn list_item_index(&self) -> i32 {
        return self.list_item_index;
    }

    pub fn block(&self) -> Option<NativeBlockModel> {
        return (self.current_block)();
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return (self.find_variable)(key);
    }

    pub fn variable_change(&self, variable: NativeVariableModel) {
        (self.variable_change)(variable);
    }

    pub fn localize(&self, key: String) -> Option<String> {
        return (self.localize)(key);
    }

    pub fn sub_block(&self, slot: String, list_item_index: i32) {
        (self.on_sub_block)(slot, list_item_index);
    }

    pub fn find_action(&self, event: String) -> Option<NativeActionModel> {
        return (self.find_action)(event);
    }

    pub async fn handle_action(
        &self,
        list_item_index: i32,
        action: Option<NativeActionModel>,
        event_type: String,
    ) {
        (self.handle_action)(list_item_index, action, event_type).await;
    }
}

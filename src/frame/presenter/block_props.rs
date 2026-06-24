use std::sync::Arc;

use crate::frame::domain::model::{NativeActionModel, NativeBlockModel, NativeVariableModel};
use crate::frame::presenter::state_manager::FrameStateManager;

pub const NONE_INDEX: i32 = -1;

#[derive(uniffi::Object)]
pub struct BlockProps {
    manager: Arc<FrameStateManager>,
    instance_name: String,
    list_item_index: i32,
    block: NativeBlockModel,
}

impl BlockProps {
    pub(super) fn new(
        manager: Arc<FrameStateManager>,
        instance_name: String,
        list_item_index: i32,
        block: NativeBlockModel,
    ) -> Self {
        return Self {
            manager,
            instance_name,
            list_item_index,
            block,
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

    pub fn block(&self) -> NativeBlockModel {
        return self.block.clone();
    }

    pub fn find_variable(&self, key: String) -> Option<NativeVariableModel> {
        return self.manager.find_variable(&key);
    }

    pub fn change_variable(&self, variable: NativeVariableModel) {
        self.manager.handle_variable(variable);
    }

    pub fn find_action(&self, event_type: String) -> Option<NativeActionModel> {
        return self.manager.find_action(&self.block.key, &event_type);
    }

    pub async fn handle_action(
        &self,
        list_item_index: i32,
        action: Option<NativeActionModel>,
        event_type: String,
    ) {
        self.manager
            .handle_action(list_item_index, action, event_type)
            .await;
    }

    pub fn sub_blocks(&self, slot: String) -> Vec<NativeBlockModel> {
        return self.manager.children(&self.block.id, &slot);
    }

    pub fn sub_block_props(&self, slot: String, list_item_index: i32) -> Vec<Arc<BlockProps>> {
        let index = if list_item_index == NONE_INDEX {
            self.list_item_index
        } else {
            list_item_index
        };
        return self
            .manager
            .children(&self.block.id, &slot)
            .into_iter()
            .map(|child| self.manager.block_props(child, index))
            .collect();
    }
}

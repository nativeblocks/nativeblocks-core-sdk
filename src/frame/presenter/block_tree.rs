use std::sync::Arc;

use crate::frame::domain::model::{NativeActionModel, NativeBlockModel};
use crate::frame::presenter::block_props::{
    BlockProps, CurrentBlock, FindAction, FindVariable, HandleAction, Localize, VariableChange,
};
use crate::frame::presenter::block_provider::BlockFinder;

const NONE_INDEX: i32 = -1;

pub(super) type FindSubBlocks = Arc<dyn Fn(&str) -> Vec<NativeBlockModel> + Send + Sync>;
pub(super) type FindActionByKey = Arc<dyn Fn(String, String) -> Option<NativeActionModel> + Send + Sync>;
pub(super) type FindBlockByKey = Arc<dyn Fn(String) -> Option<NativeBlockModel> + Send + Sync>;

#[allow(clippy::too_many_arguments)]
pub(super) fn render(
    instance_name: String,
    find_sub_blocks: FindSubBlocks,
    find_variable: FindVariable,
    variable_change: VariableChange,
    localize: Localize,
    find_action: FindActionByKey,
    find_block: FindBlockByKey,
    handle_action: HandleAction,
    block_finder: Arc<dyn BlockFinder>,
) {
    let root = (find_sub_blocks)("")
        .into_iter()
        .min_by_key(|block| block.position);
    let root = match root {
        Some(root) => root,
        None => return,
    };

    render_block(
        block_finder,
        instance_name,
        find_sub_blocks,
        find_variable,
        variable_change,
        localize,
        find_action,
        find_block,
        handle_action,
        root,
        NONE_INDEX,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_block(
    block_finder: Arc<dyn BlockFinder>,
    instance_name: String,
    find_blocks: FindSubBlocks,
    find_variable: FindVariable,
    variable_change: VariableChange,
    localize: Localize,
    find_action: FindActionByKey,
    find_block: FindBlockByKey,
    handle_action: HandleAction,
    block: NativeBlockModel,
    list_item_index: i32,
) {
    match block_finder.find_handler(block.key_type.clone()) {
        Some(handler) => {
            let key_type = block.key_type.clone();
            let props = Arc::new(build_props(
                block_finder,
                instance_name,
                find_blocks,
                find_variable,
                variable_change,
                localize,
                find_action,
                find_block,
                handle_action,
                block,
                list_item_index,
            ));
            handler.handle(key_type, props);
        }
        None => {
            block_finder.fallback(block.key_type, block.key);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn build_props(
    block_finder: Arc<dyn BlockFinder>,
    instance_name: String,
    find_blocks: FindSubBlocks,
    find_variable: FindVariable,
    variable_change: VariableChange,
    localize: Localize,
    find_action: FindActionByKey,
    find_block: FindBlockByKey,
    handle_action: HandleAction,
    block: NativeBlockModel,
    list_item_index: i32,
) -> BlockProps {
    let parent_id = block.id.clone();
    let block_key = block.key.clone();
    let sub_finder = block_finder.clone();
    let sub_instance = instance_name.clone();
    let sub_find_variable = find_variable.clone();
    let sub_variable_change = variable_change.clone();
    let sub_localize = localize.clone();
    let sub_find_action = find_action.clone();
    let sub_find_block = find_block.clone();
    let sub_handle_action = handle_action.clone();
    let on_sub_block = Arc::new(move |slot: String, index: i32| {
        let mut children: Vec<NativeBlockModel> = (find_blocks)(&parent_id)
            .into_iter()
            .filter(|block| block.slot == slot)
            .collect();
        children.sort_by_key(|block| block.position);
        for child in children {
            render_block(
                sub_finder.clone(),
                sub_instance.clone(),
                find_blocks.clone(),
                sub_find_variable.clone(),
                sub_variable_change.clone(),
                sub_localize.clone(),
                sub_find_action.clone(),
                sub_find_block.clone(),
                sub_handle_action.clone(),
                child,
                index,
            );
        }
    });

    let current_block: CurrentBlock = {
        let find_block = find_block.clone();
        let block_key = block_key.clone();
        Arc::new(move || find_block(block_key.clone()))
    };
    let find_action_bound: FindAction = {
        let block_key = block_key.clone();
        Arc::new(move |event| find_action(block_key.clone(), event))
    };

    return BlockProps::new(
        instance_name,
        list_item_index,
        current_block,
        find_variable,
        variable_change,
        localize,
        on_sub_block,
        find_action_bound,
        handle_action,
    );
}

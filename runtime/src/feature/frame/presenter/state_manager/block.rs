use crate::feature::frame::domain::model::{NativeBlockModel, NativeBlockModifierModel};
use crate::feature::frame::presenter::logging::FrameLogger;
use crate::feature::frame::presenter::state_manager::model::BlockLogEvent;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use std::collections::HashMap;

type SubKeyIndex = HashMap<String, HashMap<String, Vec<String>>>;

pub(super) fn render(
    state: &InternalState,
    logger: &FrameLogger,
) -> HashMap<String, NativeBlockModel> {
    let mut placed: Vec<&NativeBlockModel> = Vec::new();

    for block in state.base.blocks.values() {
        let required = block.scope.clone().unwrap_or_default();
        let provided = slot_scope(state, block);
        if required.is_empty() || required == provided {
            placed.push(block);
            continue;
        }
        let dropped = !provided.is_empty();
        logger.block(BlockLogEvent::ScopeMismatch {
            block_key: block.key.clone(),
            key_type: block.key_type.clone(),
            required,
            provided,
            dropped,
        });
        if !dropped {
            placed.push(block);
        }
    }

    placed.sort_by_key(|block| block.position);
    let index = sub_key_index(&placed);
    return state
        .base
        .blocks
        .values()
        .map(|block| {
            (
                block.key.clone(),
                with_children(state, block, &index, logger),
            )
        })
        .collect();
}

fn sub_key_index(placed: &[&NativeBlockModel]) -> SubKeyIndex {
    let mut index: SubKeyIndex = HashMap::new();
    for block in placed {
        index
            .entry(block.parent_key.clone())
            .or_default()
            .entry(block.slot.clone())
            .or_default()
            .push(block.key.clone());
    }
    return index;
}

fn with_children(
    state: &InternalState,
    block: &NativeBlockModel,
    index: &SubKeyIndex,
    logger: &FrameLogger,
) -> NativeBlockModel {
    let mut filled = block.clone();
    filled.sub_keys = index.get(block.key.as_str()).cloned().unwrap_or_default();
    filled.modifiers = scoped_modifiers(state, block, logger);
    return filled;
}

fn scoped_modifiers(
    state: &InternalState,
    block: &NativeBlockModel,
    logger: &FrameLogger,
) -> Vec<NativeBlockModifierModel> {
    let provided = slot_scope(state, block);
    let mut applied: Vec<NativeBlockModifierModel> = block
        .modifiers
        .iter()
        .filter(|modifier| modifier_fits(modifier, block, &provided, logger))
        .cloned()
        .collect();
    applied.sort_by_key(|modifier| modifier.position);
    return applied;
}

fn modifier_fits(
    modifier: &NativeBlockModifierModel,
    block: &NativeBlockModel,
    provided: &str,
    logger: &FrameLogger,
) -> bool {
    let required = modifier.scope.clone().unwrap_or_default();
    if required.is_empty() || required == provided {
        return true;
    }
    let dropped = !provided.is_empty();
    logger.block(BlockLogEvent::ModifierScopeMismatch {
        block_key: block.key.clone(),
        key_type: modifier.key_type.clone(),
        required,
        provided: provided.to_string(),
        dropped,
    });
    return !dropped;
}

fn slot_scope(state: &InternalState, block: &NativeBlockModel) -> String {
    let Some(parent) = state.base.blocks.get(&block.parent_key) else {
        return String::new();
    };
    let Some(slot) = parent.slots.get(&block.slot) else {
        return String::new();
    };
    return slot.scope.clone().unwrap_or_default();
}

use std::collections::{HashMap, HashSet};
use std::mem::replace;
use std::sync::Arc;

use crate::frame::domain::model::{
    NativeActionModel, NativeBlockModel, NativeFrameModel, NativeVariableModel,
};
use crate::frame::presenter::block_context::BlockObserver;

use super::{FrameState, FrameStateManager};

const VARIABLE_TYPE_STRING: &str = "STRING";

pub(super) fn build_tree(
    frame: NativeFrameModel,
    args: &HashMap<String, String>,
    globals: &HashMap<String, String>,
) -> (
    HashMap<String, NativeVariableModel>,
    HashMap<String, Vec<NativeBlockModel>>,
    Option<String>,
    HashMap<String, Vec<NativeActionModel>>,
    HashMap<String, HashSet<String>>,
    FrameState,
) {
    let variable_dependent_block_ids = buid_variable_dependent_block_ids(&frame.blocks);
    return (
        merge_variables(frame.variables, args, globals),
        frame.blocks,
        frame.root_id,
        frame.actions,
        variable_dependent_block_ids,
        FrameState::Ready {},
    );
}

fn merge_variables(
    mut variables: HashMap<String, NativeVariableModel>,
    args: &HashMap<String, String>,
    globals: &HashMap<String, String>,
) -> HashMap<String, NativeVariableModel> {
    for (key, value) in args.iter().chain(globals.iter()) {
        variables.insert(
            key.clone(),
            NativeVariableModel {
                key: key.clone(),
                value: value.clone(),
                variable_type: VARIABLE_TYPE_STRING.to_string(),
            },
        );
    }
    return variables;
}

fn block_variables(block: &NativeBlockModel) -> Vec<String> {
    let mut keys: Vec<String> = block
        .data
        .values()
        .filter(|data| !data.value.is_empty())
        .map(|data| data.value.clone())
        .collect();
    if !block.visibility.is_empty() {
        keys.push(block.visibility.clone());
    }
    return keys;
}

fn buid_variable_dependent_block_ids(blocks: &HashMap<String, Vec<NativeBlockModel>>) -> HashMap<String, HashSet<String>> {
    let mut var_deps: HashMap<String, HashSet<String>> = HashMap::new();
    for block in blocks.values().flatten() {
        for variable_key in block_variables(block) {
            var_deps.entry(variable_key).or_default().insert(block.id.clone());
        }
    }
    return var_deps;
}

impl FrameStateManager {

    pub(crate) fn block_by_key(&self, key: &str) -> Option<NativeBlockModel> {
        let state = self.state.lock().unwrap();
        let found = state.blocks.values().flatten().find(|block| block.key == key);
        return found.cloned();
    }

    pub(crate) fn block_by_id(&self, id: &str) -> Option<NativeBlockModel> {
        let state = self.state.lock().unwrap();
        let found = state.blocks.values().flatten().find(|block| block.id == id);
        return found.cloned();
    }

    pub(crate) fn children(&self, parent_id: &str, slot: &str) -> Vec<String> {
        let state = self.state.lock().unwrap();
        let children = state.blocks.get(parent_id);
        if children.is_none() {
            return Vec::new(); // unknown parent or no children
        }
        return children
            .unwrap()
            .iter()
            .filter(|child| child.slot == slot)
            .map(|child| child.id.clone())
            .collect();
    }

    pub(crate) fn variable(&self, key: &str) -> Option<NativeVariableModel> {
        return self.state.lock().unwrap().variables.get(key).cloned();
    }

    pub(crate) fn translate(&self, key: &str) -> Option<String> {
        let localization = self.localization.lock().unwrap().clone();
        return localization.and_then(|manager| manager.translate(key.to_string()));
    }
}

impl FrameStateManager {
    pub(crate) fn mutate_variable(&self, variable: NativeVariableModel) {
        let mut state = self.state.lock().unwrap();

        let current = state.variables.get(&variable.key);
        if current == Some(&variable) {
            return;
        }
        let previous_value = current.map(|current| current.value.clone());

        let dependent_block_ids: Vec<String> = if let Some(ids) = state.variable_dependent_block_ids.get(&variable.key) {
            ids.iter().cloned().collect()
        } else {
            Vec::new()
        };

        state.variables.insert(variable.key.clone(), variable.clone());

        drop(state);
        self.logger.variable_changed(
            previous_value.as_deref(),
            &variable.key,
            &variable.value,
            &variable.variable_type,
        );
        self.notify_block_observers(&dependent_block_ids);
    }

    pub(crate) fn mutate_block(&self, new_block: NativeBlockModel) {
        if new_block.id.is_empty() {
            return;
        }
        let id = new_block.id.clone();

        let mut state = self.state.lock().unwrap();

        let siblings = state.blocks.get_mut(&new_block.parent_id);
        if siblings.is_none() {
            return;
        }
        let siblings = siblings.unwrap();

        let index = siblings.iter().position(|sibling| sibling.id == id);
        if index.is_none() {
            return;
        }
        let index = index.unwrap();

        if siblings[index] == new_block {
            return;
        }

        let position_changed = siblings[index].position != new_block.position;
        let new_variable_keys = block_variables(&new_block);
        let current_block = replace(&mut siblings[index], new_block);

        if position_changed {
            siblings.sort_by_key(|sibling| sibling.position);
        }

        for variable_key in block_variables(&current_block) {
            if let Some(dependents) = state.variable_dependent_block_ids.get_mut(&variable_key) {
                dependents.remove(&id);
            }
        }
        for variable_key in new_variable_keys {
            state.variable_dependent_block_ids.entry(variable_key).or_default().insert(id.clone());
        }

        drop(state);
        self.logger.block_changed(&current_block.key, &current_block.key_type);
        self.notify_block_observers(&[id]);
    }
}

impl FrameStateManager {
    pub(crate) fn subscribe_block(&self, block_id: &str, index: i32, observer: Arc<dyn BlockObserver>) {
        let mut state = self.state.lock().unwrap();
        let subscribers = state.block_observers.entry(block_id.to_string()).or_default();

        let existing = subscribers.iter_mut().find(|(existing_index, _)| *existing_index == index);
        if let Some((_, existing_observer)) = existing {
            *existing_observer = observer;
        } else {
            subscribers.push((index, observer));
        }
    }

    pub(crate) fn unsubscribe_block(&self, block_id: &str, index: i32) {
        let mut state = self.state.lock().unwrap();
        if let Some(subscribers) = state.block_observers.get_mut(block_id) {
            subscribers.retain(|(existing_index, _)| *existing_index != index);
            if subscribers.is_empty() {
                state.block_observers.remove(block_id);
            }
        }
    }

    pub(super) fn notify_block_observers(&self, block_ids: &[String]) {
        if block_ids.is_empty() {
            return;
        }

        let state = self.state.lock().unwrap();
        let mut observers_to_notify: Vec<Arc<dyn BlockObserver>> = Vec::new();
        for block_id in block_ids {
            if let Some(subscribers) = state.block_observers.get(block_id) {
                for (_, observer) in subscribers {
                    observers_to_notify.push(observer.clone());
                }
            }
        }
        drop(state);

        for observer in observers_to_notify {
            observer.on_invalidate();
        }
    }
}

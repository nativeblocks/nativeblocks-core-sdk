use crate::feature::frame::domain::model::{
    NativeBlockModel, NativeBlockPropertyModel, NativeFrameModel, NativeVariableModel,
};
use crate::feature::frame::presenter::state_manager::model::{
    FrameDiff, FrameFull, RenderingState,
};
use std::collections::HashMap;
use std::sync::Arc;

const VARIABLE_TYPE_STRING: &str = "STRING";

pub(super) struct InternalState {
    base: Arc<NativeFrameModel>,
    state: RenderingState,
    root_key: Option<String>,
    variables: HashMap<String, NativeVariableModel>,
    blocks: HashMap<String, NativeBlockModel>,
}

#[derive(Debug, Clone)]
pub(crate) struct FrameSnapshot {
    variables: HashMap<String, NativeVariableModel>,
    blocks: HashMap<String, NativeBlockModel>,
}

impl InternalState {
    pub(super) fn fresh() -> Self {
        return Self::empty(RenderingState::Loading {});
    }

    pub(super) fn error(message: String) -> Self {
        return Self::empty(RenderingState::Error { message });
    }

    fn empty(state: RenderingState) -> Self {
        return Self {
            base: Arc::new(NativeFrameModel {
                checksum: None,
                variables: HashMap::new(),
                blocks: HashMap::new(),
                root_key: None,
                actions: HashMap::new(),
            }),
            state,
            root_key: None,
            variables: HashMap::new(),
            blocks: HashMap::new(),
        };
    }

    pub(super) fn ready(
        frame: Arc<NativeFrameModel>,
        args: &HashMap<String, String>,
        globals: &HashMap<String, String>,
    ) -> Self {
        let variables = merge_variables(frame.variables.clone(), args, globals);
        let root_key = frame.root_key.clone();
        return Self {
            base: frame,
            state: RenderingState::Ready {},
            root_key,
            variables,
            blocks: HashMap::new(),
        };
    }

    pub(super) fn to_frame_full(&self) -> FrameFull {
        let index = self.sub_key_index();
        let blocks = self
            .blocks()
            .map(|block| (block.key.clone(), self.bake(block, &index)))
            .collect();
        return FrameFull {
            state: self.state.clone(),
            root_key: self.root_key.clone(),
            blocks,
            variables: self.variables.clone(),
            actions: self.base.actions.clone(),
            restored: false,
        };
    }

    pub(super) fn snapshot(&self) -> FrameSnapshot {
        return FrameSnapshot {
            variables: self.variables.clone(),
            blocks: self.blocks.clone(),
        };
    }

    pub(super) fn restore(&mut self, snapshot: &FrameSnapshot) {
        for (key, variable) in &snapshot.variables {
            if self.variables.contains_key(key) {
                self.variables.insert(key.clone(), variable.clone());
            }
        }
        for (key, block) in &snapshot.blocks {
            if self.base.blocks.contains_key(key) {
                self.blocks.insert(key.clone(), block.clone());
            }
        }
    }

    pub(super) fn change_variable(&mut self, key: &str, value: String) -> Option<FrameDiff> {
        let diff = self.variable_diff(key, value)?;
        self.variables.extend(diff.variables.clone());
        return Some(diff);
    }

    pub(super) fn change_block_property(
        &mut self,
        block_key: String,
        property_key: String,
        value_mobile: String,
        value_tablet: String,
        value_desktop: String,
    ) -> Option<FrameDiff> {
        let block = self.block_of(&block_key)?;
        if block.key == "" {
            return None;
        }
        let mut updated = block.clone();
        let current_prop = updated.properties.get(&property_key)?.clone();
        let new_prop = NativeBlockPropertyModel {
            value_mobile,
            value_tablet,
            value_desktop,
            ..current_prop
        };
        updated.properties.insert(property_key, new_prop);
        self.blocks.insert(updated.key.clone(), updated);

        let index = self.sub_key_index();
        let mut blocks = HashMap::new();
        if let Some(block) = self.block_of(&block_key) {
            blocks.insert(block_key.clone(), self.bake(block, &index));
        }
        return Some(FrameDiff {
            variables: HashMap::new(),
            blocks,
        });
    }

    fn block_of(&self, key: &str) -> Option<&NativeBlockModel> {
        return self.blocks.get(key).or_else(|| self.base.blocks.get(key));
    }

    fn blocks(&self) -> impl Iterator<Item = &NativeBlockModel> {
        return self
            .base
            .blocks
            .iter()
            .filter(move |(key, _)| !self.blocks.contains_key(key.as_str()))
            .map(|(_, block)| block)
            .chain(self.blocks.values());
    }

    fn bake(
        &self,
        block: &NativeBlockModel,
        index: &HashMap<String, HashMap<String, Vec<String>>>,
    ) -> NativeBlockModel {
        let mut baked = block.clone();
        baked.sub_keys = index.get(block.key.as_str()).cloned().unwrap_or_default();
        return baked;
    }

    fn sub_key_index(&self) -> HashMap<String, HashMap<String, Vec<String>>> {
        let mut children_by_parent: HashMap<&str, Vec<&NativeBlockModel>> = HashMap::new();
        for block in self.blocks() {
            children_by_parent
                .entry(block.parent_key.as_str())
                .or_default()
                .push(block);
        }
        let mut index = HashMap::with_capacity(children_by_parent.len());
        for (parent, mut children) in children_by_parent {
            children.sort_by_key(|block| block.position);
            index.insert(parent.to_string(), group_by_slot(children));
        }
        return index;
    }

    fn variable_diff(&self, key: &str, value: String) -> Option<FrameDiff> {
        let existing = self.variables.get(key)?;
        if existing.value == value {
            return None;
        }
        let updated = NativeVariableModel {
            key: key.to_string(),
            value,
            variable_type: existing.variable_type.clone(),
        };
        let index = self.sub_key_index();
        return Some(FrameDiff {
            variables: HashMap::from([(key.to_string(), updated)]),
            blocks: self.blocks_using(key, &index),
        });
    }

    fn blocks_using(
        &self,
        variable_key: &str,
        index: &HashMap<String, HashMap<String, Vec<String>>>,
    ) -> HashMap<String, NativeBlockModel> {
        return self
            .blocks()
            .filter(|block| block.data.values().any(|data| data.value == variable_key))
            .map(|block| (block.key.clone(), self.bake(block, index)))
            .collect();
    }
}

fn group_by_slot(ordered_children: Vec<&NativeBlockModel>) -> HashMap<String, Vec<String>> {
    let mut by_slot: HashMap<String, Vec<String>> = HashMap::new();
    for child in ordered_children {
        by_slot
            .entry(child.slot.clone())
            .or_default()
            .push(child.key.clone());
    }
    return by_slot;
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

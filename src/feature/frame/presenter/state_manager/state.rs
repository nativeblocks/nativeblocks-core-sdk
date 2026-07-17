use crate::feature::frame::domain::model::{
    NativeBlockModel, NativeFrameModel, NativeVariableModel,
};
use crate::feature::frame::presenter::state_manager::model::{
    FrameDiff, FrameFull, RenderingState,
};
use std::collections::HashMap;

const VARIABLE_TYPE_STRING: &str = "STRING";

pub(super) struct InternalState {
    pub(super) frame: FrameFull,
}

impl InternalState {
    pub(super) fn fresh() -> Self {
        return Self {
            frame: FrameFull {
                state: RenderingState::Loading {},
                root_key: None,
                blocks: HashMap::new(),
                variables: HashMap::new(),
                actions: HashMap::new(),
            },
        };
    }

    pub(super) fn error(message: String) -> Self {
        return Self {
            frame: FrameFull {
                state: RenderingState::Error { message },
                root_key: None,
                blocks: HashMap::new(),
                variables: HashMap::new(),
                actions: HashMap::new(),
            },
        };
    }

    pub(super) fn ready(
        frame: &NativeFrameModel,
        args: &HashMap<String, String>,
        globals: &HashMap<String, String>,
    ) -> Self {
        return Self {
            frame: FrameFull {
                state: RenderingState::Ready {},
                root_key: frame.root_key.clone(),
                blocks: frame.blocks.clone(),
                actions: frame.actions.clone(),
                variables: merge_variables(frame.variables.clone(), args, globals),
            },
        };
    }

    pub(super) fn change_variable(&mut self, key: &str, value: String) -> Option<FrameDiff> {
        let diff = self.variable_diff(key, value)?;
        self.frame.variables.extend(diff.variables.clone());
        return Some(diff);
    }

    pub(super) fn change_block(&mut self, block: &NativeBlockModel) -> Option<FrameDiff> {
        let diff = self.block_diff(block)?;
        self.frame.blocks.extend(diff.blocks.clone());
        return Some(diff);
    }

    fn variable_diff(&self, key: &str, value: String) -> Option<FrameDiff> {
        let existing = self.frame.variables.get(key)?;
        if existing.value == value {
            return None;
        }
        let updated = NativeVariableModel {
            key: key.to_string(),
            value,
            variable_type: existing.variable_type.clone(),
        };
        return Some(FrameDiff {
            variables: HashMap::from([(key.to_string(), updated)]),
            blocks: self.blocks_using(key),
        });
    }

    fn blocks_using(&self, variable_key: &str) -> HashMap<String, NativeBlockModel> {
        return self
            .frame
            .blocks
            .values()
            .filter(|block| block.data.values().any(|data| data.value == variable_key))
            .map(|block| (block.key.clone(), block.clone()))
            .collect();
    }

    fn block_diff(&self, block: &NativeBlockModel) -> Option<FrameDiff> {
        match self.frame.blocks.get(&block.key) {
            None => return None,
            Some(existing) if existing == block => return None,
            Some(_) => {}
        }
        return Some(FrameDiff {
            variables: HashMap::new(),
            blocks: HashMap::from([(block.key.clone(), block.clone())]),
        });
    }
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

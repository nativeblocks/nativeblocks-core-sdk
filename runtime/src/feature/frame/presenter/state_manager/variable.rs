use crate::feature::frame::domain::model::NativeVariableModel;
use crate::feature::frame::presenter::state_manager::model::FrameDiff;
use crate::feature::frame::presenter::state_manager::state::InternalState;
use std::collections::HashMap;

pub(super) fn merge(
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
                variable_type: "STRING".to_string(),
            },
        );
    }
    return variables;
}

pub(super) fn is_host_provided(state: &InternalState, key: &str) -> bool {
    return !state.base.variables.contains_key(key) && state.variables.contains_key(key);
}

pub(super) fn change(state: &mut InternalState, key: &str, value: String) -> Option<FrameDiff> {
    let existing = state.variables.get(key)?;
    if existing.value == value {
        return None;
    }
    let updated = NativeVariableModel {
        key: key.to_string(),
        value,
        variable_type: existing.variable_type.clone(),
    };
    state.variables.insert(key.to_string(), updated.clone());
    return Some(FrameDiff {
        variables: HashMap::from([(key.to_string(), updated)]),
    });
}

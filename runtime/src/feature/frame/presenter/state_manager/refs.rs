use crate::feature::frame::domain::model::NativeVariableModel;
use std::collections::HashMap;

const REF_OPEN: &str = "{{var:";
const REF_CLOSE: &str = "}}";
pub(super) fn has_references(value: &str, key: &str) -> bool {
    let needle = format!("{REF_OPEN}{key}{REF_CLOSE}");
    return value.contains(&needle);
}

pub(super) fn resolved_model(
    variable: &NativeVariableModel,
    variables: &HashMap<String, NativeVariableModel>,
) -> NativeVariableModel {
    return NativeVariableModel {
        key: variable.key.clone(),
        value: resolve(&variable.value, variables),
        variable_type: variable.variable_type.clone(),
    };
}

pub(super) fn resolved_all(
    variables: &HashMap<String, NativeVariableModel>,
) -> HashMap<String, NativeVariableModel> {
    return variables
        .iter()
        .map(|(key, variable)| (key.clone(), resolved_model(variable, variables)))
        .collect();
}

fn resolve(value: &str, variables: &HashMap<String, NativeVariableModel>) -> String {
    if !value.contains(REF_OPEN) {
        return value.to_string();
    }

    let mut resolved = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find(REF_OPEN) {
        let after_open = &rest[start + REF_OPEN.len()..];
        let Some(end) = after_open.find(REF_CLOSE) else {
            break;
        };
        let key = &after_open[..end];
        resolved.push_str(&rest[..start]);
        match variables.get(key) {
            Some(target) => resolved.push_str(&target.value),
            None => resolved.push_str(&rest[start..start + REF_OPEN.len() + end + REF_CLOSE.len()]),
        }
        rest = &after_open[end + REF_CLOSE.len()..];
    }
    resolved.push_str(rest);
    return resolved;
}

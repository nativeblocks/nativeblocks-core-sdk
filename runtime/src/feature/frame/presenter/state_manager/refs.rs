use crate::feature::frame::domain::model::NativeVariableModel;
use crate::library::template::{self, Template};
use std::collections::HashMap;

const VAR: &str = "var";

pub(super) fn has_references(value: &str, key: &str) -> bool {
    return value.contains(&template::reference(VAR, key));
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
    if !template::has(value, VAR) {
        return value.to_string();
    }
    return Template::new()
        .register(VAR, |key| {
            variables.get(key).map(|target| target.value.clone())
        })
        .resolve(value);
}

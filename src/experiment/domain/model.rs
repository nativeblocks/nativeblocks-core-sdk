use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeExperimentModel {
    pub value: String,
    pub variable_type: String,
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeFrameModel {
    pub checksum: Option<String>,
    pub variables: HashMap<String, NativeVariableModel>,
    pub blocks: HashMap<String, NativeBlockModel>,
    pub root_key: Option<String>,
    pub actions: HashMap<String, Vec<NativeActionModel>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeVariableModel {
    pub key: String,
    pub value: String,
    pub variable_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockModel {
    pub id: String,
    pub parent_id: String,
    pub parent_key: String,
    pub version: i32,
    pub slot: String,
    pub key_type: String,
    pub key: String,
    pub visibility: String,
    pub position: i32,
    pub data: HashMap<String, NativeBlockDataModel>,
    pub properties: HashMap<String, NativeBlockPropertyModel>,
    pub slots: HashMap<String, NativeBlockSlotModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockPropertyModel {
    pub key: String,
    pub value_mobile: String,
    pub value_tablet: String,
    pub value_desktop: String,
    pub property_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockDataModel {
    pub key: String,
    pub value: String,
    pub data_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockSlotModel {
    pub slot: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionModel {
    pub id: String,
    pub key: String,
    pub event: String,
    pub triggers: Vec<NativeActionTriggerModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerModel {
    pub name: String,
    pub id: String,
    pub parent_id: String,
    pub version: i32,
    pub key_type: String,
    pub then: NativeActionTriggerThen,
    pub properties: HashMap<String, NativeActionTriggerPropertyModel>,
    pub data: HashMap<String, NativeActionTriggerDataModel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, uniffi::Enum)]
pub enum NativeActionTriggerThen {
    Success,
    Failure,
    Next,
    End,
}

impl NativeActionTriggerThen {
    pub fn from_string(value: &str) -> Self {
        return match value {
            "SUCCESS" => NativeActionTriggerThen::Success,
            "FAILURE" => NativeActionTriggerThen::Failure,
            "NEXT" => NativeActionTriggerThen::Next,
            _ => NativeActionTriggerThen::End,
        };
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerPropertyModel {
    pub key: String,
    pub value: String,
    pub property_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerDataModel {
    pub key: String,
    pub value: String,
    pub data_type: String,
}

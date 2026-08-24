#![allow(deprecated)]

use rkyv::{Archive, Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize)]
pub struct NativeFrameModel {
    pub checksum: Option<String>,
    pub root_key: Option<String>,
    pub variables: HashMap<String, NativeVariableModel>,
    pub blocks: HashMap<String, NativeBlockModel>,
    pub actions: HashMap<String, Vec<NativeActionModel>>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeVariableModel {
    pub key: String,
    pub value: String,
    pub variable_type: String,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockModel {
    pub id: String,
    pub parent_id: String,
    pub parent_key: String,
    pub version: i32,
    pub slot: String,
    pub key_type: String,
    pub key: String,
    pub scope: Option<String>,
    pub visibility: String,
    pub position: i32,
    pub data: HashMap<String, NativeBlockDataModel>,
    #[deprecated(note = "Properties are being replaced by data; declare block arguments as data.")]
    pub properties: HashMap<String, NativeBlockPropertyModel>,
    pub slots: HashMap<String, NativeBlockSlotModel>,
    pub modifiers: Vec<NativeBlockModifierModel>,
    pub sub_keys: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockModifierModel {
    pub key_type: String,
    pub position: i32,
    pub scope: Option<String>,
    pub data: HashMap<String, NativeBlockModifierDataModel>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockModifierDataModel {
    pub key: String,
    pub value: String,
    pub data_type: String,
}

#[deprecated(note = "Properties are being replaced by data.")]
#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockPropertyModel {
    pub key: String,
    pub value_mobile: String,
    pub value_tablet: String,
    pub value_desktop: String,
    pub property_type: String,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockDataModel {
    pub key: String,
    pub value: String,
    pub data_type: String,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockSlotModel {
    pub slot: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionModel {
    pub id: String,
    pub key: String,
    pub event: String,
    pub triggers: Vec<NativeActionTriggerModel>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerModel {
    pub name: String,
    pub id: String,
    pub parent_id: String,
    pub version: i32,
    pub key_type: String,
    pub event: String,
    pub scope: Option<String>,
    #[deprecated(
        note = "Properties are being replaced by data; declare action arguments as data."
    )]
    pub properties: HashMap<String, NativeActionTriggerPropertyModel>,
    pub data: HashMap<String, NativeActionTriggerDataModel>,
    pub events: HashMap<String, NativeActionTriggerEventModel>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerEventModel {
    pub event: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerPropertyModel {
    pub key: String,
    pub value: String,
    pub property_type: String,
}

#[derive(Debug, Clone, PartialEq, Archive, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerDataModel {
    pub key: String,
    pub value: String,
    pub data_type: String,
}

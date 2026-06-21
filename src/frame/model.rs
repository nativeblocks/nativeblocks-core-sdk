use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::config::ProjectConfigGateway;

#[derive(Debug, Clone)]
pub(crate) struct FrameSyncRequest {
    pub endpoint_frame: ProjectConfigGateway,
    pub endpoint_frame_production: ProjectConfigGateway,
    pub endpoint_frame_production_checksum: ProjectConfigGateway,
    pub graphql_endpoint: String,
    pub install_id: String,
    pub route: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeVariableModel {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

impl NativeVariableModel {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        value_type: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            value_type: value_type.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct NativeBlockModel {
    pub id: String,
    pub parent_id: String,
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
#[serde(rename_all = "camelCase")]
pub struct NativeBlockPropertyModel {
    pub key: String,
    pub value_mobile: String,
    pub value_tablet: String,
    pub value_desktop: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeBlockDataModel {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: String,
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
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "NEXT")]
    Next,
    #[serde(rename = "END")]
    End,
}

impl NativeActionTriggerThen {
    pub fn as_str(self) -> &'static str {
        match self {
            NativeActionTriggerThen::Success => "SUCCESS",
            NativeActionTriggerThen::Failure => "FAILURE",
            NativeActionTriggerThen::Next => "NEXT",
            NativeActionTriggerThen::End => "END",
        }
    }

    pub fn from_then(value: &str) -> Self {
        match value {
            "SUCCESS" => NativeActionTriggerThen::Success,
            "FAILURE" => NativeActionTriggerThen::Failure,
            "NEXT" => NativeActionTriggerThen::Next,
            _ => NativeActionTriggerThen::End,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerPropertyModel {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, uniffi::Record)]
pub struct NativeActionTriggerDataModel {
    pub key: String,
    pub value: String,
    #[serde(rename = "type")]
    pub value_type: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub(crate) struct NativeFrameModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, NativeVariableModel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<HashMap<String, Vec<NativeActionModel>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<HashMap<String, NativeBlockModel>>,
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct NativeFrameState {
    pub is_loading: bool,
    pub is_success: bool,
    pub development_mode: bool,
    pub error: String,
}

impl NativeFrameState {
    pub fn initial(development_mode: bool) -> Self {
        Self {
            is_loading: true,
            is_success: false,
            development_mode,
            error: String::new(),
        }
    }

    pub fn success(development_mode: bool) -> Self {
        Self {
            is_loading: false,
            is_success: true,
            development_mode,
            error: String::new(),
        }
    }

    pub fn error(development_mode: bool, error: impl Into<String>) -> Self {
        Self {
            is_loading: false,
            is_success: false,
            development_mode,
            error: error.into(),
        }
    }
}

use crate::feature::frame::{NativeActionModel, NativeBlockModel, NativeVariableModel};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum RenderingState {
    Loading {},
    Ready {},
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FrameFull {
    pub state: RenderingState,
    pub root_key: Option<String>,
    pub blocks: HashMap<String, NativeBlockModel>,
    pub variables: HashMap<String, NativeVariableModel>,
    pub actions: HashMap<String, Vec<NativeActionModel>>,
    pub restored: bool,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FrameDiff {
    pub variables: HashMap<String, NativeVariableModel>,
    pub blocks: HashMap<String, NativeBlockModel>,
}

#[derive(Debug, Clone, PartialEq, uniffi::Enum)]
pub enum FrameChangeType {
    Full { frame: FrameFull },
    Diff { frame: FrameDiff },
}

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum ActionLogEvent {
    EventIgnored { event: String },
    EventTriggered { event: String, action_key: String },
    TriggerExecuted { name: String, key_type: String, then: String },
    TriggerFallback { key_type: String, name: String },
}

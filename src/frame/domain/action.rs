use std::collections::HashMap;

use crate::frame::domain::model::{
    NativeActionTriggerModel, NativeActionTriggerThen, NativeBlockModel, NativeVariableModel,
};

/// keyType of the built-in scripting action, executed in core.
pub const SCRIPT_KEY_TYPE: &str = "SCRIPT";

/// The context the engine hands a host action handler. Carries the trigger plus
/// whole-state snapshots (the coarse projection of the engine's variable/block
/// state) so the handler can read without chatty per-key FFI crossings.
#[derive(Debug, Clone, uniffi::Record)]
pub struct ActionContext {
    pub instance_name: String,
    pub list_item_index: i32,
    pub key_type: String,
    pub trigger: NativeActionTriggerModel,
    pub variables: HashMap<String, NativeVariableModel>,
    pub blocks: HashMap<String, NativeBlockModel>,
}

/// What a host action handler returns. `then` selects which child triggers run
/// next (the SUCCESS / FAILURE / NEXT branch); the engine applies the variable
/// and block changes before continuing.
#[derive(Debug, Clone, uniffi::Record)]
pub struct ActionResult {
    pub then: NativeActionTriggerThen,
    pub variable_changes: Vec<NativeVariableModel>,
    pub block_changes: Vec<NativeBlockModel>,
}

impl ActionResult {
    pub fn next() -> Self {
        Self {
            then: NativeActionTriggerThen::Next,
            variable_changes: Vec::new(),
            block_changes: Vec::new(),
        }
    }
}

/// Host-implemented action (navigation, dialog, network, contractor, app
/// custom). The engine resolves a trigger's `keyType` to a registered handler
/// and awaits it; the result drives the rest of the trigger graph. Async so
/// host actions can do real work (e.g. network) without blocking the engine.
#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NativeActionHandler: Send + Sync {
    async fn handle(&self, context: ActionContext) -> ActionResult;
}

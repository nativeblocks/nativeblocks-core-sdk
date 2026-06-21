use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rquickjs::{Coerced, Context, Function, Object, Runtime};
use tokio::sync::watch;

use crate::common::config::SdkConfig;
use crate::common::logger::{LoggerEventLevel, NativeLoggerProvider, keys};
use crate::frame::domain::action::ActionResult;
use crate::frame::domain::model::{NativeBlockModel, NativeBlockPropertyModel, NativeVariableModel};

const SCRIPT_TIMEOUT: Duration = Duration::from_millis(2000);

/// State handle the script functions read and mutate. Holds clones of the
/// engine's `watch` senders (sharing the same channels) plus the logging
/// context, so `updateVariable`/`updateBlockProperties` feed engine state and
/// emit the same change logs as direct engine mutations.
#[derive(Clone)]
pub(crate) struct ScriptBridge {
    pub(crate) variables: watch::Sender<HashMap<String, NativeVariableModel>>,
    pub(crate) blocks: watch::Sender<HashMap<String, NativeBlockModel>>,
    pub(crate) logger: Arc<Mutex<NativeLoggerProvider>>,
    pub(crate) config: SdkConfig,
    pub(crate) development_mode: bool,
    pub(crate) route: String,
}

impl ScriptBridge {
    fn find_variable(&self, key: &str) -> Option<NativeVariableModel> {
        self.variables.borrow().get(key).cloned()
    }

    fn change_variable(&self, variable: NativeVariableModel) {
        let key = variable.key.clone();
        let value = variable.value.clone();
        let value_type = variable.value_type.clone();
        let previous = self.variables.borrow().get(&key).map(|v| v.value.clone());
        self.variables.send_modify(|m| {
            m.insert(key.clone(), variable);
        });

        let is_changed = previous.as_ref().is_some_and(|p| *p != value);
        let message = if is_changed {
            format!(
                "Variable changed: {key} changed from '{}' to '{value}'",
                previous.clone().unwrap_or_default()
            )
        } else {
            format!("Variable changed: {key} set to '{value}'")
        };
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::VARIABLE_UPDATED.to_string(),
        );
        params.insert(keys::parameter::KEY.to_string(), key);
        params.insert(keys::parameter::NEW_VALUE.to_string(), value);
        params.insert(keys::parameter::VARIABLE_TYPE.to_string(), value_type);
        if is_changed {
            params.insert(
                keys::parameter::PREVIOUS_VALUE.to_string(),
                previous.unwrap_or_default(),
            );
        }
        self.log(self.level(), keys::tag::VARIABLE_CHANGE, message, params);
    }

    fn find_block(&self, key: &str) -> Option<NativeBlockModel> {
        self.blocks.borrow().get(key).cloned()
    }

    fn change_block(&self, block: NativeBlockModel) {
        let key = block.key.clone();
        let key_type = block.key_type.clone();
        self.blocks.send_modify(|m| {
            m.insert(key.clone(), block);
        });
        let mut params = HashMap::new();
        params.insert(
            keys::parameter::STATE.to_string(),
            keys::state::BLOCK_UPDATED.to_string(),
        );
        params.insert(keys::parameter::BLOCK_KEY.to_string(), key.clone());
        params.insert(keys::parameter::KEY_TYPE.to_string(), key_type.clone());
        self.log(
            self.level(),
            keys::tag::BLOCK_CHANGE,
            format!("Block updated: {key}[{key_type}]"),
            params,
        );
    }

    fn level(&self) -> LoggerEventLevel {
        if self.development_mode {
            LoggerEventLevel::Debug
        } else {
            LoggerEventLevel::Info
        }
    }

    fn log(
        &self,
        level: LoggerEventLevel,
        event: &str,
        message: String,
        mut params: HashMap<String, String>,
    ) {
        params.insert(keys::parameter::FRAME_ROUTE.to_string(), self.route.clone());
        if let Ok(provider) = self.logger.lock() {
            provider.dispatch(&self.config, level, event, message, params);
        }
    }
}

/// Runs the trigger's `script` property. Mirrors `NativeScriptAction`: replace
/// `{{index}}`, evaluate in a sandboxed QuickJS context with `getVariable` /
/// `updateVariable` / `updateBlockProperties`, then continue the NEXT branch.
/// Script errors are swallowed (as in Kotlin) — the graph always proceeds.
pub(crate) fn evaluate(
    bridge: &ScriptBridge,
    trigger: &crate::frame::domain::model::NativeActionTriggerModel,
    index: i32,
) -> ActionResult {
    let script = trigger
        .properties
        .get("script")
        .map(|p| p.value.clone())
        .unwrap_or_default();
    if script.is_empty() {
        return ActionResult::next();
    }
    let processed = script.replace("{{index}}", &index.to_string());
    let _ = run(bridge.clone(), processed);
    ActionResult::next()
}

fn run(bridge: ScriptBridge, script: String) -> rquickjs::Result<()> {
    let runtime = Runtime::new()?;
    let deadline = Instant::now() + SCRIPT_TIMEOUT;
    runtime.set_interrupt_handler(Some(Box::new(move || Instant::now() >= deadline)));
    let context = Context::full(&runtime)?;

    context.with(|ctx| -> rquickjs::Result<()> {
        let globals = ctx.globals();

        let get_bridge = bridge.clone();
        globals.set(
            "getVariable",
            Function::new(ctx.clone(), move |key: Coerced<String>| {
                get_bridge.find_variable(&key.0).map(|v| v.value)
            })?,
        )?;

        let update_bridge = bridge.clone();
        globals.set(
            "updateVariable",
            Function::new(
                ctx.clone(),
                move |key: Coerced<String>, value: Coerced<String>| {
                    if let Some(variable) = update_bridge.find_variable(&key.0) {
                        let casted = cast(&value.0, &variable.value_type);
                        update_bridge.change_variable(NativeVariableModel::new(
                            variable.key,
                            casted,
                            variable.value_type,
                        ));
                    }
                },
            )?,
        )?;

        let block_bridge = bridge.clone();
        globals.set(
            "updateBlockProperties",
            Function::new(
                ctx.clone(),
                move |block_key: Coerced<String>, property_key: Coerced<String>, values: Object| {
                    update_block_properties(&block_bridge, &block_key.0, &property_key.0, &values);
                },
            )?,
        )?;

        // Strip dangerous globals (mirrors NativeScriptAction's security step).
        let _ = ctx.eval::<(), _>(
            "delete globalThis.Function; delete globalThis.eval; \
             delete globalThis.console; delete globalThis.require; \
             delete globalThis.module; delete globalThis.process;",
        );

        let _ = ctx.eval::<rquickjs::Value, _>(script);
        Ok(())
    })
}

fn update_block_properties(
    bridge: &ScriptBridge,
    block_key: &str,
    property_key: &str,
    values: &Object,
) {
    let Some(mut block) = bridge.find_block(block_key) else {
        return;
    };
    let Some(current) = block.properties.get(property_key).cloned() else {
        return;
    };
    let updated = NativeBlockPropertyModel {
        key: current.key.clone(),
        value_mobile: get_optional(values, "mobile").unwrap_or(current.value_mobile),
        value_tablet: get_optional(values, "tablet").unwrap_or(current.value_tablet),
        value_desktop: get_optional(values, "desktop").unwrap_or(current.value_desktop),
        value_type: current.value_type,
    };
    block.properties.insert(property_key.to_string(), updated);
    bridge.change_block(block);
}

fn get_optional(values: &Object, key: &str) -> Option<String> {
    values
        .get::<_, Option<Coerced<String>>>(key)
        .ok()
        .flatten()
        .map(|c| c.0)
}

fn cast(value: &str, value_type: &str) -> String {
    match value_type.to_uppercase().as_str() {
        "INT" => value.parse::<i32>().map(|n| n.to_string()).unwrap_or_default(),
        "DOUBLE" => value.parse::<f64>().map(|n| n.to_string()).unwrap_or_default(),
        "LONG" => value.parse::<i64>().map(|n| n.to_string()).unwrap_or_default(),
        "FLOAT" => value.parse::<f32>().map(|n| n.to_string()).unwrap_or_default(),
        "BOOLEAN" => match value {
            "true" => "true".to_string(),
            "false" => "false".to_string(),
            _ => String::new(),
        },
        _ => value.to_string(),
    }
}

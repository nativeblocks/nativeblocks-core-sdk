use std::sync::Arc;
use std::time::{Duration, Instant};

use rquickjs::{CatchResultExt, Coerced, Context, Function, Object, Runtime};

#[derive(Debug, Clone, PartialEq, Eq, uniffi::Record)]
pub struct ScriptResult {
    pub value: Option<String>,
    pub error: Option<String>,
}

#[uniffi::export(with_foreign)]
pub trait ScriptBridge: Send + Sync {
    fn get_variable(&self, key: String) -> Option<String>;
    fn update_variable(&self, key: String, value: String);
    fn update_block_property(
        &self,
        block_key: String,
        property_key: String,
        mobile: Option<String>,
        tablet: Option<String>,
        desktop: Option<String>,
    );
}

#[derive(uniffi::Object)]
pub struct ScriptEngine;

#[uniffi::export]
impl ScriptEngine {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        return Arc::new(Self);
    }

    pub fn evaluate(&self, script: String, bridge: Arc<dyn ScriptBridge>, timeout_ms: u64) -> ScriptResult {
        return evaluate(script, bridge, timeout_ms);
    }
}

fn evaluate(script: String, bridge: Arc<dyn ScriptBridge>, timeout_ms: u64) -> ScriptResult {
    let runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => return error_result(error.to_string()),
    };

    let timeout = Duration::from_millis(timeout_ms);
    let start = Instant::now();
    runtime.set_interrupt_handler(Some(Box::new(move || start.elapsed() >= timeout)));

    let context = match Context::full(&runtime) {
        Ok(context) => context,
        Err(error) => return error_result(error.to_string()),
    };

    return context.with(|ctx| {
        let _ = ctx.eval::<(), _>("delete globalThis.eval; delete globalThis.Function;");

        if let Err(error) = bind_host_functions(&ctx, &bridge) {
            return error_result(error.to_string());
        }

        match ctx.eval::<Coerced<String>, _>(script).catch(&ctx) {
            Ok(value) => ScriptResult {
                value: Some(value.0),
                error: None,
            },
            Err(error) => {
                let message = if start.elapsed() >= timeout {
                    "Execution timed out".to_string()
                } else {
                    error.to_string()
                };
                error_result(message)
            }
        }
    });
}

fn bind_host_functions(ctx: &rquickjs::Ctx, bridge: &Arc<dyn ScriptBridge>) -> rquickjs::Result<()> {
    let globals = ctx.globals();

    let get = bridge.clone();
    globals.set(
        "getVariable",
        Function::new(ctx.clone(), move |key: String| get.get_variable(key))?,
    )?;

    let update = bridge.clone();
    globals.set(
        "updateVariable",
        Function::new(ctx.clone(), move |key: String, value: Coerced<String>| {
            update.update_variable(key, value.0)
        })?,
    )?;

    let block = bridge.clone();
    globals.set(
        "updateBlockProperties",
        Function::new(
            ctx.clone(),
            move |block_key: String, property_key: String, values: Object| {
                let mobile: Option<String> = values.get("mobile").unwrap_or(None);
                let tablet: Option<String> = values.get("tablet").unwrap_or(None);
                let desktop: Option<String> = values.get("desktop").unwrap_or(None);
                block.update_block_property(block_key, property_key, mobile, tablet, desktop)
            },
        )?,
    )?;

    return Ok(());
}

fn error_result(message: String) -> ScriptResult {
    return ScriptResult {
        value: None,
        error: Some(message),
    };
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

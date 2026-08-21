use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::plugin::script::ScriptEngine;
use rquickjs::{CatchResultExt, Coerced, Context, Ctx, Exception, Function, Object, Runtime};

const PRELUDE: &str = include_str!("nb-common.js");

const SEAL: &str = r#"(function () {
    var seal = function (fn) {
        Object.defineProperty(Object.getPrototypeOf(fn), "constructor", {
            value: undefined,
            writable: false,
            configurable: false,
        });
    };
    seal(function () {});
    seal(function* () {});
    seal(async function () {});
    seal(async function* () {});
    delete globalThis.eval;
    delete globalThis.Function;
})();"#;

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

#[uniffi::export]
impl ScriptEngine {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        return Arc::new(Self);
    }

    pub fn evaluate(
        &self,
        script: String,
        bridge: Arc<dyn ScriptBridge>,
        timeout_ms: u64,
    ) -> ScriptResult {
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
        if let Err(error) = bind_host_functions(&ctx, &bridge, timeout_ms) {
            return error_result(error.to_string());
        }

        if let Err(error) = ctx.eval::<(), _>(PRELUDE).catch(&ctx) {
            return error_result(error.to_string());
        }

        if let Err(error) = ctx.eval::<(), _>(SEAL).catch(&ctx) {
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

fn bind_host_functions(
    ctx: &Ctx,
    bridge: &Arc<dyn ScriptBridge>,
    timeout_ms: u64,
) -> rquickjs::Result<()> {
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

    globals.set(
        "__hostNow",
        Function::new(ctx.clone(), || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|since_epoch| since_epoch.as_millis() as f64)
                .unwrap_or(0.0)
        })?,
    )?;

    globals.set(
        "__hostDiagnostic",
        Function::new(ctx.clone(), |message: Coerced<String>| {
            eprintln!("[nativeblocks script] {}", message.0);
        })?,
    )?;

    globals.set(
        "__hostDelay",
        Function::new(ctx.clone(), host_delay(timeout_ms))?,
    )?;

    return Ok(());
}

fn host_delay(timeout_ms: u64) -> impl for<'js> Fn(Ctx<'js>, Coerced<f64>) -> rquickjs::Result<()> {
    return move |ctx, milliseconds: Coerced<f64>| {
        let requested = milliseconds.0;
        if !requested.is_finite() || requested <= 0.0 {
            return Ok(());
        }
        if requested > timeout_ms as f64 {
            return Err(Exception::throw_message(
                &ctx,
                "Delay exceeds the execution timeout",
            ));
        }
        thread::sleep(Duration::from_millis(requested as u64));
        return Ok(());
    };
}

fn error_result(message: String) -> ScriptResult {
    return ScriptResult {
        value: None,
        error: Some(message),
    };
}

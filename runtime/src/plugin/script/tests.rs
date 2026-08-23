use super::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Default)]
struct MockBridge {
    variables: Mutex<HashMap<String, String>>,
}

impl ScriptBridge for MockBridge {
    fn get_variable(&self, key: String) -> Option<String> {
        return self.variables.lock().unwrap().get(&key).cloned();
    }
    fn update_variable(&self, key: String, value: String) {
        self.variables.lock().unwrap().insert(key, value);
    }
}

fn run(script: &str) -> ScriptResult {
    let engine = ScriptEngine::new();
    return engine.evaluate(script.to_string(), Arc::new(MockBridge::default()), 2000);
}

#[test]
fn arithmetic() {
    assert_eq!(run("2 + 2").value.as_deref(), Some("4"));
}

#[test]
fn if_else() {
    assert_eq!(
        run("if (10 > 5) { 10 * 2; } else { 5 * 2; }")
            .value
            .as_deref(),
        Some("20")
    );
}

#[test]
fn json() {
    assert_eq!(
        run(r#"JSON.stringify({name:"Dao", age:30})"#)
            .value
            .as_deref(),
        Some(r#"{"name":"Dao","age":30}"#)
    );
}

#[test]
fn eval_is_blocked() {
    assert!(run("eval('2 + 2')").error.is_some());
}

#[test]
fn function_is_blocked() {
    assert!(run("new Function('return 5')()").error.is_some());
}

#[test]
fn infinite_loop_times_out() {
    let engine = ScriptEngine::new();
    let result = engine.evaluate(
        "while (true) {}".to_string(),
        Arc::new(MockBridge::default()),
        200,
    );
    assert_eq!(result.error.as_deref(), Some("Execution timed out"));
}

#[test]
fn variable_round_trip() {
    let engine = ScriptEngine::new();
    let bridge = Arc::new(MockBridge::default());
    bridge
        .variables
        .lock()
        .unwrap()
        .insert("count".into(), "3".into());
    engine.evaluate(
        r#"updateVariable("count", getVariable("count") * 2);"#.to_string(),
        bridge.clone(),
        2000,
    );
    assert_eq!(
        bridge
            .variables
            .lock()
            .unwrap()
            .get("count")
            .map(String::as_str),
        Some("6")
    );
}

#[test]
fn prelude_is_preloaded() {
    assert_eq!(
        run(r#"__cat(__str(null), __join(__map([1, 2], (x) => __numAdd(x, 1)), "-"))"#)
            .value
            .as_deref(),
        Some("2-3")
    );
    assert_eq!(run("__div(1, 0)").value.as_deref(), Some("0"));
    assert_eq!(run("__get(null, 'a')").value.as_deref(), Some("null"));
}

#[test]
fn host_clock_is_bound() {
    let millis: f64 = run("__now()").value.unwrap().parse().unwrap();
    assert!(millis > 1_600_000_000_000.0, "got {millis}");
}

#[test]
fn diagnostic_is_bound() {
    assert_eq!(run(r#"__diag("hello"); "ok""#).value.as_deref(), Some("ok"));
}

#[test]
fn delay_sleeps() {
    let started = Instant::now();
    assert_eq!(
        run(r#"__delay(150); "done""#).value.as_deref(),
        Some("done")
    );
    assert!(started.elapsed() >= Duration::from_millis(150));
}

#[test]
fn delay_longer_than_the_timeout_errors() {
    let engine = ScriptEngine::new();
    let started = Instant::now();
    let result = engine.evaluate(
        r#"__delay(5000); "done""#.to_string(),
        Arc::new(MockBridge::default()),
        100,
    );
    assert!(
        result
            .error
            .unwrap()
            .contains("Delay exceeds the execution timeout")
    );
    assert!(started.elapsed() < Duration::from_millis(1000));
}

#[test]
fn delay_ignores_junk() {
    let started = Instant::now();
    assert_eq!(
        run(r#"__delay(-5); __delay("nope"); __delay(null); "ok""#)
            .value
            .as_deref(),
        Some("ok")
    );
    assert!(started.elapsed() < Duration::from_millis(100));
}

#[test]
fn compiler_is_unreachable() {
    for vector in [
        r#"eval("2 + 2")"#,
        r#"new Function("return 2 + 2")()"#,
        r#"(function () {}).constructor("return 2 + 2")()"#,
        r#"Object.getPrototypeOf(function () {}).constructor("return 2 + 2")()"#,
        r#"(function* () {}).constructor("return 2 + 2")"#,
        r#"(async function () {}).constructor("return 2 + 2")"#,
        r#"(async function* () {}).constructor("return 2 + 2")"#,
        r#"__str.constructor("return 2 + 2")()"#,
    ] {
        assert!(run(vector).error.is_some(), "reachable: {vector}");
    }
}

#[test]
fn dynamic_import_cannot_run_code() {
    let engine = ScriptEngine::new();
    let bridge = Arc::new(MockBridge::default());
    engine.evaluate(
        r#"import("data:text/javascript,export default 4").then(function () { updateVariable("k", "PWNED"); });"#
            .to_string(),
        bridge.clone(),
        500,
    );
    assert!(bridge.variables.lock().unwrap().is_empty());
}

#[test]
fn sealing_leaves_ordinary_constructors_alone() {
    assert_eq!(
        run("({}).constructor === Object").value.as_deref(),
        Some("true")
    );
    assert_eq!(
        run("[].constructor === Array").value.as_deref(),
        Some("true")
    );
}

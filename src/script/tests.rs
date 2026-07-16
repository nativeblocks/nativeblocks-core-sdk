use super::*;
use std::collections::HashMap;
use std::sync::Mutex;

type BlockUpdate = (
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
);

#[derive(Default)]
struct MockBridge {
    variables: Mutex<HashMap<String, String>>,
    block_updates: Mutex<Vec<BlockUpdate>>,
}

impl ScriptBridge for MockBridge {
    fn get_variable(&self, key: String) -> Option<String> {
        return self.variables.lock().unwrap().get(&key).cloned();
    }
    fn update_variable(&self, key: String, value: String) {
        self.variables.lock().unwrap().insert(key, value);
    }
    fn update_block_property(
        &self,
        block_key: String,
        property_key: String,
        mobile: Option<String>,
        tablet: Option<String>,
        desktop: Option<String>,
    ) {
        self.block_updates
            .lock()
            .unwrap()
            .push((block_key, property_key, mobile, tablet, desktop));
    }
}

fn run(script: &str) -> ScriptResult {
    return evaluate(script.to_string(), Arc::new(MockBridge::default()), 2000);
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
    let result = evaluate(
        "while (true) {}".to_string(),
        Arc::new(MockBridge::default()),
        200,
    );
    assert_eq!(result.error.as_deref(), Some("Execution timed out"));
}

#[test]
fn variable_round_trip() {
    let bridge = Arc::new(MockBridge::default());
    bridge
        .variables
        .lock()
        .unwrap()
        .insert("count".into(), "3".into());
    evaluate(
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
fn block_property_update() {
    let bridge = Arc::new(MockBridge::default());
    evaluate(
        r#"updateBlockProperties("blk", "text", { mobile: "M", desktop: "D" });"#.to_string(),
        bridge.clone(),
        2000,
    );
    let updates = bridge.block_updates.lock().unwrap();
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].0, "blk");
    assert_eq!(updates[0].1, "text");
    assert_eq!(updates[0].2.as_deref(), Some("M"));
    assert_eq!(updates[0].3, None);
    assert_eq!(updates[0].4.as_deref(), Some("D"));
}

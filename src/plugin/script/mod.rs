mod engine;

pub use engine::{ScriptBridge, ScriptResult};

#[derive(uniffi::Object)]
struct ScriptEngine;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

use std::sync::Arc;

use crate::frame::domain::model::NativeActionTriggerModel;
use crate::frame::presenter::state_manager::FrameStateManager;
use crate::script::ScriptEngine;
use crate::script::ScriptBridge;

#[cfg(feature = "script-quickjs")]
const SCRIPT_TIMEOUT_MS: u64 = 2000;

pub(crate) fn run_script(
    manager: &Arc<FrameStateManager>,
    index: i32,
    trigger: &NativeActionTriggerModel,
) {
    #[cfg(feature = "script-quickjs")]
    {
        let script = trigger
            .properties
            .get("script")
            .map(|property| property.value.clone())
            .unwrap_or_default();
        if script.is_empty() {
            return;
        }
        let processed = script.replace("{{index}}", &index.to_string());
        let bridge: Arc<dyn ScriptBridge> = Arc::new(ManagerScriptBridge { manager: manager.clone() });
        let _ = ScriptEngine::new().evaluate(processed, bridge, SCRIPT_TIMEOUT_MS);
    }
}

#[cfg(feature = "script-quickjs")]
fn cast_value(value: &str, variable_type: &str) -> String {
    return match variable_type.to_uppercase().as_str() {
        "INT" => value
            .parse::<i32>()
            .map(|parsed| parsed.to_string())
            .unwrap_or_default(),
        "LONG" => value
            .parse::<i64>()
            .map(|parsed| parsed.to_string())
            .unwrap_or_default(),
        "DOUBLE" => value
            .parse::<f64>()
            .map(|parsed| parsed.to_string())
            .unwrap_or_default(),
        "FLOAT" => value
            .parse::<f32>()
            .map(|parsed| parsed.to_string())
            .unwrap_or_default(),
        "BOOLEAN" => match value {
            "true" => "true".to_string(),
            "false" => "false".to_string(),
            _ => String::new(),
        },
        _ => value.to_string(),
    };
}

#[cfg(feature = "script-quickjs")]
struct ManagerScriptBridge {
    manager: Arc<FrameStateManager>,
}

#[cfg(feature = "script-quickjs")]
impl crate::script::ScriptBridge for ManagerScriptBridge {
    fn get_variable(&self, key: String) -> Option<String> {
        return self.manager.variable(&key).map(|variable| variable.value);
    }

    fn update_variable(&self, key: String, value: String) {
        if let Some(mut variable) = self.manager.variable(&key) {
            variable.value = cast_value(&value, &variable.variable_type);
            self.manager.mutate_variable(variable);
        }
    }

    fn update_block_property(
        &self,
        block_key: String,
        property_key: String,
        mobile: Option<String>,
        tablet: Option<String>,
        desktop: Option<String>,
    ) {
        if let Some(mut block) = self.manager.block_by_key(&block_key) {
            if let Some(property) = block.properties.get_mut(&property_key) {
                if let Some(value) = mobile {
                    property.value_mobile = value;
                }
                if let Some(value) = tablet {
                    property.value_tablet = value;
                }
                if let Some(value) = desktop {
                    property.value_desktop = value;
                }
                self.manager.mutate_block(block);
            }
        }
    }
}

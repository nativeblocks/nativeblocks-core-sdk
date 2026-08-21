import Foundation
import NativeblocksRuntimeFFI

/// NativeScriptAction provides scripting capabilities within the Nativeblocks system,
/// executing JavaScript through the engine's QuickJS runtime (Rust, via uniffi)
/// instead of JavaScriptCore. The script API is unchanged, so existing scripts
/// keep working.
///
/// ## Configuration
///
/// **Properties:**
/// - `script`: The JavaScript code to execute.
///
/// ## Dynamic Placeholders
/// - `{{index}}`: Replaced with the current list item index (useful in list/loop contexts).
///
/// ## Built-in Functions
/// The following functions are available within the script:
///
/// - `getVariable(variableKey)`: Returns the current value of a variable.
///
/// - `updateVariable(variableKey, value)`: Updates a variable with the given key and value.
///   The value is automatically cast to match the variable's type.
///
/// - `updateBlockProperties(blockKey, propertyKey, { mobile, tablet, desktop })`:
///   Updates a block's property per screen size. Only the provided (non-null)
///   values are updated.
///
/// Example (variables):
/// ```javascript
/// const count = getVariable("count");
/// let result = count;
/// if (count >= 1) {
///     result = count - 1;
/// }
/// updateVariable("count", result);
/// ```
///
/// Example (block properties):
/// ```javascript
/// updateBlockProperties("myBlock", "text", { mobile: "Mobile Text" });
/// updateBlockProperties("myBlock", "backgroundColor", { mobile: "#FF0000", tablet: "#00FF00", desktop: "#0000FF" });
/// ```
///
/// ## Blocked Features (for security)
/// - `eval`, `Function` (removed from the QuickJS global scope); execution is
///   interrupted after the timeout.
///
/// ## Variable Type Casting
/// Values passed to `updateVariable` are automatically cast to match the target
/// variable's type (STRING, INT, DOUBLE, LONG, FLOAT, BOOLEAN).
internal final class NativeScriptAction: INativeAction {

    static let KEY_TYPE = "SCRIPT"
    private static let DEFAULT_TIMEOUT_MS: UInt64 = 2000

    func handle(actionContext: ActionContext) {
        Task.detached {
            let trigger = actionContext.trigger
            let script = trigger?.properties["script"]?.value ?? ""

            if !script.isEmpty {
                let processedScript = script.replacingOccurrences(
                    of: "{{index}}",
                    with: String(actionContext.listItemIndex)
                )
                Self.evaluateScript(processedScript, actionContext: actionContext)
            }
            if let trigger {
                actionContext.onHandleNextTrigger(trigger)
            }
        }
    }

    private static func evaluateScript(_ script: String, actionContext: ActionContext) {
        let bridge = ScriptBridgeAdapter(actionContext: actionContext)
        _ = ScriptEngine().evaluate(script: script, bridge: bridge, timeoutMs: DEFAULT_TIMEOUT_MS)
    }

    /// Converts a string value to a specified type.
    ///
    /// - Parameters:
    ///   - value: The value to cast.
    ///   - type: The target type (INT, DOUBLE, LONG, FLOAT, BOOLEAN, STRING).
    /// - Returns: The converted value as a string, or an empty string if conversion fails.
    fileprivate static func cast(value: String?, type: String?) -> String? {
        guard let value, let type else { return value }
        let trimmed = value.trimmingCharacters(in: .whitespacesAndNewlines)

        switch type.uppercased() {
        case "INT": return Int32(trimmed).map { "\($0)" } ?? ""
        case "DOUBLE": return Double(trimmed).map { "\($0)" } ?? ""
        case "LONG": return Int64(trimmed).map { "\($0)" } ?? ""
        case "FLOAT": return Float(trimmed).map { "\($0)" } ?? ""
        case "BOOLEAN": return Bool(trimmed.lowercased()).map { "\($0)" } ?? ""
        default: return value
        }
    }
}

/// Exposes the frame's variables and blocks to the QuickJS runtime.
private final class ScriptBridgeAdapter: ScriptBridge, @unchecked Sendable {

    private let actionContext: ActionContext

    init(actionContext: ActionContext) {
        self.actionContext = actionContext
    }

    func getVariable(key: String) -> String? {
        return actionContext.onFindVariable(key)?.value
    }

    func updateVariable(key: String, value: String) {
        guard let variable = actionContext.onFindVariable(key) else { return }
        let castedValue = NativeScriptAction.cast(value: value, type: variable.type) ?? value
        actionContext.onUpdateVariable(variable.copy(value: castedValue))
    }

    func updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        mobile: String?,
        tablet: String?,
        desktop: String?
    ) {
        guard let block = actionContext.onFindBlock(blockKey),
            let currentProperty = block.properties[propertyKey]
        else { return }

        actionContext.onUpdateBlockProperties(
            blockKey,
            propertyKey,
            mobile ?? currentProperty.valueMobile,
            tablet ?? currentProperty.valueTablet,
            desktop ?? currentProperty.valueDesktop
        )
    }
}

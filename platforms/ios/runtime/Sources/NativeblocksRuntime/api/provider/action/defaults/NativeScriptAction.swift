import Foundation
import NativeblocksRuntimeFFI

internal let SCRIPT_NEXT_EVENT = "NEXT"

/// NativeScriptAction provides scripting capabilities within the Nativeblocks system,
/// executing JavaScript through the engine's QuickJS runtime
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
                actionContext.onHandleEvent(SCRIPT_NEXT_EVENT)
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

}

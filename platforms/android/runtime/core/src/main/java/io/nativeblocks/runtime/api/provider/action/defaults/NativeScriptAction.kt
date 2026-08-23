@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.api.provider.action.defaults

import io.nativeblocks.runtime.api.provider.action.ActionContext
import io.nativeblocks.runtime.api.provider.action.INativeAction
import io.nativeblocks.runtime.ffi.ScriptBridge
import io.nativeblocks.runtime.ffi.ScriptEngine
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

/**
 * NativeScriptAction provides scripting capabilities within the Nativeblocks system,
 * executing JavaScript through the engine's QuickJS runtime.
 *
 * ## Configuration
 *
 * **Properties:**
 * - `script`: The JavaScript code to execute.
 *
 * ## Dynamic Placeholders
 * - `{{index}}`: Replaced with the current list item index (useful in list/loop contexts).
 *
 * ## Built-in Functions
 * The following functions are available within the script:
 *
 * - `getVariable(variableKey)`: Returns the current value of a variable.
 *
 * - `updateVariable(variableKey, value)`: Updates a variable with the given key and
 *   value. The value is automatically cast to match the variable's type.
 *
 *
 * Example (variables):
 * ```javascript
 * const count = getVariable("count");
 * let result = count;
 * if (count >= 1) {
 *     result = count - 1;
 * }
 * updateVariable("count", result);
 * ```
 *
 * ## Blocked Features (for security)
 * - `eval`, `Function` (removed from the QuickJS global scope); execution is
 *   interrupted after the timeout.
 *
 * ## Variable Type Casting
 * Values passed to `updateVariable` are automatically cast to match the target
 * variable's type (STRING, INT, DOUBLE, LONG, FLOAT, BOOLEAN).
 */
internal const val SCRIPT_NEXT_EVENT: String = "NEXT"

internal class NativeScriptAction : INativeAction {

    companion object {
        const val KEY_TYPE = "SCRIPT"
        private val DEFAULT_TIMEOUT_MS = 2000uL
    }

    override fun handle(actionContext: ActionContext) {
        actionContext.coroutineScope.launch {
            val trigger = actionContext.trigger
            val script = trigger?.properties?.get("script")?.value.orEmpty()

            if (script.isNotEmpty()) {
                val processedScript =
                    script.replace("{{index}}", actionContext.listItemIndex.toString())
                evaluateScript(processedScript, actionContext)
            }
            actionContext.onHandleEvent(SCRIPT_NEXT_EVENT)
        }
    }

    private suspend fun evaluateScript(script: String, actionContext: ActionContext) {
        withContext(Dispatchers.IO) {
            val bridge = object : ScriptBridge {
                override fun getVariable(key: String): String? {
                    return actionContext.onFindVariable(key)?.value
                }

                override fun updateVariable(key: String, value: String) {
                    val variable = actionContext.onFindVariable(key) ?: return
                    val castedValue = cast(value, variable.type) ?: value
                    actionContext.onUpdateVariable(variable.copy(value = castedValue))
                }

            }

            ScriptEngine().use { engine ->
                engine.evaluate(script, bridge, DEFAULT_TIMEOUT_MS)
            }
        }
    }

    /**
     * Converts a string value to a specified type.
     *
     * @param value The value to cast.
     * @param type The target type (INT, DOUBLE, LONG, FLOAT, BOOLEAN, STRING).
     * @return The converted value as a string, or empty string if conversion fails.
     */
    private fun cast(value: String?, type: String?): String? {
        return when (type?.uppercase()) {
            "INT" -> value?.toIntOrNull()?.toString() ?: ""
            "DOUBLE" -> value?.toDoubleOrNull()?.toString() ?: ""
            "LONG" -> value?.toLongOrNull()?.toString() ?: ""
            "FLOAT" -> value?.toFloatOrNull()?.toString() ?: ""
            "BOOLEAN" -> value?.toBooleanStrictOrNull()?.toString() ?: ""
            else -> value
        }
    }
}

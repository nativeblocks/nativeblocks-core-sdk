package io.nativeblocks.runtime.api.provider.action.defaults

import io.nativeblocks.runtime.api.provider.action.ActionProps
import io.nativeblocks.runtime.api.provider.action.INativeAction
import io.nativeblocks.runtime.ffi.ScriptBridge
import io.nativeblocks.runtime.ffi.ScriptEngine
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

/**
 * NativeScriptAction provides scripting capabilities within the Nativeblocks system,
 * executing JavaScript through the engine's QuickJS runtime (Rust, via uniffi)
 * instead of the old Rhino interpreter. The script API is unchanged, so existing
 * scripts keep working.
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
 * - `updateBlockProperties(blockKey, propertyKey, { mobile, tablet, desktop })`:
 *   Updates a block's property per screen size. Only the provided (non-null)
 *   values are updated.
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
 * Example (block properties):
 * ```javascript
 * updateBlockProperties("myBlock", "text", { mobile: "Mobile Text" });
 * updateBlockProperties("myBlock", "backgroundColor", { mobile: "#FF0000", tablet: "#00FF00", desktop: "#0000FF" });
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
internal class NativeScriptAction : INativeAction {

    companion object {
        const val KEY_TYPE = "SCRIPT"
        private val DEFAULT_TIMEOUT_MS = 2000uL
    }

    override fun handle(actionProps: ActionProps) {
        actionProps.coroutineScope.launch {
            val trigger = actionProps.trigger
            val script = trigger?.properties?.get("script")?.value.orEmpty()

            if (script.isNotEmpty()) {
                val processedScript =
                    script.replace("{{index}}", actionProps.listItemIndex.toString())
                evaluateScript(processedScript, actionProps)
            }
            trigger?.let { actionProps.onHandleNextTrigger(it) }
        }
    }

    private suspend fun evaluateScript(script: String, actionProps: ActionProps) {
        withContext(Dispatchers.IO) {
            val bridge = object : ScriptBridge {
                override fun getVariable(key: String): String? {
                    return actionProps.onFindVariable(key)?.value
                }

                override fun updateVariable(key: String, value: String) {
                    val variable = actionProps.onFindVariable(key) ?: return
                    val castedValue = cast(value, variable.type) ?: value
                    actionProps.onChangeVariable(variable.copy(value = castedValue))
                }

                override fun updateBlockProperty(
                    blockKey: String,
                    propertyKey: String,
                    mobile: String?,
                    tablet: String?,
                    desktop: String?,
                ) {
                    val block = actionProps.onFindBlock(blockKey) ?: return
                    val properties = block.properties.toMutableMap()
                    val currentProperty = properties[propertyKey] ?: return
                    actionProps.onChangeBlockProperties(
                        blockKey,
                        propertyKey,
                        mobile ?: currentProperty.valueMobile,
                        tablet ?: currentProperty.valueTablet,
                        desktop ?: currentProperty.valueDesktop,
                    )
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

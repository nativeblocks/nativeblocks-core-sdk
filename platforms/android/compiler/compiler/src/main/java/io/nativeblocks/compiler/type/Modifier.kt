package io.nativeblocks.compiler.type

/**
 * Annotation to define a Native Modifier within the Nativeblocks system.
 *
 * A modifier decorates the block it is attached to. It contributes styling or behavior
 * and never renders content of its own.
 *
 * @property keyType The type of key associated with the modifier.
 * @property name The name of the modifier.
 * @property description A description of the modifier.
 * @property scope The scope this modifier requires from the slot its host block sits in.
 * @property version The version of the modifier. Defaults to 1.
 * @property versionName The version name of the modifier. Defaults to an empty string.
 * @property deprecated Indicates if the modifier is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.FUNCTION)
annotation class Modifier(
    val keyType: String,
    val name: String,
    val description: String,
    val scope: String = "",
    val version: Int = 1,
    val versionName: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to define a data argument for a Native Modifier.
 *
 * Modifier arguments are always data, so they can be bound to variables and changed at runtime.
 *
 * @property description A brief description of the data argument.
 * @property deprecated Indicates if the data argument is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 * @property defaultValue The default value for the data argument, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class ModifierData(
    val description: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue: String = ""
)

/**
 * Annotation to define an event binding for a Native Modifier.
 *
 * The parameter's own name is the event name, and triggers are filed under it.
 *
 * @property description A brief description of the event binding.
 * @property dataBindings Data keys this event hands back, in the order the callback reports them.
 * @property deprecated Indicates if the event binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class ModifierEvent(
    val description: String = "",
    val dataBindings: Array<String> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)


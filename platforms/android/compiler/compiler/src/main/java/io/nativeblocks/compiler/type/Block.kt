package io.nativeblocks.compiler.type

/**
 * Alias for the index of a block.
 */
typealias BlockIndex = Int

/**
 * Annotation to define a Native Block within the Nativeblocks system.
 *
 * @property keyType The type of key associated with the block.
 * @property name The name of the block.
 * @property description A description of the block.
 * @property version The version of the block. Defaults to 1.
 * @property versionName The version Name of the block. Defaults an empty string.
 * @property deprecated Indicates if the block is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.FUNCTION)
annotation class Block(
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
 * Annotation to define a property for a Native Block.
 *
 * @property description A brief description of the property.
 * @property valuePicker Specifies the type of value picker to use for this property.
 * @property valuePickerGroup Defines the grouping of the value picker for UI purposes.
 * @property valuePickerOptions Array of selectable options, if applicable.
 * @property deprecated Indicates if the property is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 * @property defaultValue The default value for the property, if applicable.
 */
@Deprecated("Properties are being replaced by data; declare block arguments with @BlockData.")
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class BlockProp(
    val description: String = "",
    val valuePicker: BlockValuePicker = BlockValuePicker.TEXT_INPUT,
    val valuePickerGroup: BlockValuePickerPosition = BlockValuePickerPosition(
        text = "General"
    ),
    val valuePickerOptions: Array<BlockValuePickerOption> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue :String = ""
)

/**
 * Annotation to define data binding for a Native Block.
 *
 * @property description A brief description of the data binding.
 * @property deprecated Indicates if the data binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 * @property defaultValue The default value for the data binding, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class BlockData(
    val description: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue :String = ""
)

/**
 * Annotation to define an event binding for a Native Block.
 *
 * @property description A brief description of the event binding.
 * @property dataBindings Data keys this event hands back, in the order the callback reports them.
 * @property deprecated Indicates if the event binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class BlockEvent(
    val description: String = "",
    val dataBindings: Array<String> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to define a slot for a Native Block.
 *
 * @property description A brief description of the slot.
 * @property scope The scope this slot hands to the blocks inside it.
 * @property dataBindings Data keys this slot hands back, in the order the callback reports them.
 * @property deprecated Indicates if the slot is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class BlockSlot(
    val description: String = "",
    val scope: String = "",
    val dataBindings: Array<String> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to define options for a value picker in Native Blocks.
 *
 * @property id Unique identifier for the option.
 * @property text Display text for the option.
 */
annotation class BlockValuePickerOption(
    val id: String,
    val text: String
)

/**
 * Annotation to define the grouping position of a value picker in the UI.
 *
 * @property text The text label for the group.
 */
annotation class BlockValuePickerPosition(
    val text: String,
)

/**
 * Enum representing the types of value pickers available for Native Block properties.
 * Defines the UI components used to input or select values for properties.
 */
enum class BlockValuePicker {
    TEXT_INPUT,
    TEXT_AREA_INPUT,
    NUMBER_INPUT,
    DROPDOWN,
    COMBOBOX_INPUT,
    COLOR_PICKER;
}
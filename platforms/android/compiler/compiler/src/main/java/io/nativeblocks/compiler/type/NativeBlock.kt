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
annotation class NativeBlock(
    val keyType: String,
    val name: String,
    val description: String,
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
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class NativeBlockProp(
    val description: String = "",
    val valuePicker: NativeBlockValuePicker = NativeBlockValuePicker.TEXT_INPUT,
    val valuePickerGroup: NativeBlockValuePickerPosition = NativeBlockValuePickerPosition(
        text = "General"
    ),
    val valuePickerOptions: Array<NativeBlockValuePickerOption> = [],
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
annotation class NativeBlockData(
    val description: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue :String = ""
)

/**
 * Annotation to define an event binding for a Native Block.
 *
 * @property description A brief description of the event binding.
 * @property dataBinding Array of data bindings for the event.
 * @property deprecated Indicates if the event binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class NativeBlockEvent(
    val description: String = "",
    val dataBinding: Array<String> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to define a slot for a Native Block.
 *
 * @property description A brief description of the slot.
 * @property deprecated Indicates if the slot is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class NativeBlockSlot(
    val description: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to define options for a value picker in Native Blocks.
 *
 * @property id Unique identifier for the option.
 * @property text Display text for the option.
 */
annotation class NativeBlockValuePickerOption(
    val id: String,
    val text: String
)

/**
 * Annotation to define the grouping position of a value picker in the UI.
 *
 * @property text The text label for the group.
 */
annotation class NativeBlockValuePickerPosition(
    val text: String,
)

/**
 * Enum representing the types of value pickers available for Native Block properties.
 * Defines the UI components used to input or select values for properties.
 */
enum class NativeBlockValuePicker {
    TEXT_INPUT,
    TEXT_AREA_INPUT,
    NUMBER_INPUT,
    DROPDOWN,
    COMBOBOX_INPUT,
    COLOR_PICKER;
}
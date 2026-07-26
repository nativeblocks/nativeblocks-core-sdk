package io.nativeblocks.compiler.type

/**
 * Annotation to define a Native Action within the Nativeblocks system.
 *
 * @property keyType The type of key associated with the action.
 * @property name The name of the action.
 * @property description A description of the action.
 * @property version The version of the action. Defaults to 1.
 * @property versionName The version name of the action. Defaults to an empty string.
 * @property deprecated Indicates if the action is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.CLASS)
annotation class NativeAction(
    val keyType: String,
    val name: String,
    val description: String,
    val version: Int = 1,
    val versionName: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Annotation to mark a class as a parameter holder for Native Actions.
 */
@Target(AnnotationTarget.CLASS)
annotation class NativeActionParameter

/**
 * Annotation to mark a function as a Native Action handler.
 */
@Target(AnnotationTarget.FUNCTION)
annotation class NativeActionFunction

/**
 * Annotation to define a property for a Native Action.
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
annotation class NativeActionProp(
    val description: String = "",
    val valuePicker: NativeActionValuePicker = NativeActionValuePicker.TEXT_INPUT,
    val valuePickerGroup: NativeActionValuePickerPosition = NativeActionValuePickerPosition(
        text = "General"
    ),
    val valuePickerOptions: Array<NativeActionValuePickerOption> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue :String = ""
)

/**
 * Annotation to define data binding for a Native Action.
 *
 * @property description A brief description of the data binding.
 * @property deprecated Indicates if the data binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 * @property defaultValue The default value for the data binding, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class NativeActionData(
    val description: String = "",
    val deprecated: Boolean = false,
    val deprecatedReason: String = "",
    val defaultValue :String = ""
)

/**
 * Annotation to define an event binding for a Native Action.
 *
 * @property then Specifies the behavior after the event is triggered.
 * @property description A brief description of the event binding.
 * @property dataBinding Array of data bindings for the event.
 * @property deprecated Indicates if the event binding is deprecated.
 * @property deprecatedReason Reason for deprecation, if applicable.
 */
@Target(AnnotationTarget.VALUE_PARAMETER)
annotation class NativeActionEvent(
    val then: Then = Then.END,
    val description: String = "",
    val dataBinding: Array<String> = [],
    val deprecated: Boolean = false,
    val deprecatedReason: String = ""
)

/**
 * Enum representing the next step after an event.
 */
enum class Then {
    SUCCESS,
    FAILURE,
    NEXT,
    END;
}

/**
 * Annotation to define options for a value picker in Native Actions.
 *
 * @property id Unique identifier for the option.
 * @property text Display text for the option.
 */
annotation class NativeActionValuePickerOption(
    val id: String,
    val text: String
)

/**
 * Annotation to define the grouping position of a value picker in the UI.
 *
 * @property text The text label for the group.
 */
annotation class NativeActionValuePickerPosition(
    val text: String,
)

/**
 * Enum representing the types of value pickers available for Native Action properties.
 * Defines the UI components used to input or select values for properties.
 */
enum class NativeActionValuePicker {
    TEXT_INPUT,
    TEXT_AREA_INPUT,
    NUMBER_INPUT,
    DROPDOWN,
    COLOR_PICKER,
    SCRIPT_AREA_INPUT;
}
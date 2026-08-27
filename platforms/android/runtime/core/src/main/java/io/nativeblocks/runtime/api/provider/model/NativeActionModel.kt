@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.api.provider.model

/**
 * Represents an action within in Nativeblocks, including its event triggers
 * and associated metadata.
 */
data class NativeActionModel(
    /**
     * Unique identifier of the action.
     */
    val id: String,

    /**
     * Key associated with the action.
     */
    val key: String,

    /**
     * Event that triggers the action.
     */
    val event: String,

    /**
     * List of triggers that define the sequence and conditions for executing the action.
     */
    val triggers: List<NativeActionTriggerModel>,
)

/**
 * Represents a trigger for an action, including its properties, data, and hierarchical sub-triggers.
 */
data class NativeActionTriggerModel(
    /**
     * Name of the trigger.
     */
    val name: String,

    /**
     * Unique identifier of the trigger.
     */
    val id: String,

    /**
     * Identifier of the parent trigger to establish hierarchy.
     */
    val parentId: String,

    /**
     * version of the trigger.
     */
    val version: Int,

    /**
     * Type of the key associated with the trigger.
     */
    val keyType: String,

    /**
     * Event of the parent this trigger is filed under.
     */
    val event: String,

    /**
     * Scope this trigger requires from the event it sits under, null when it accepts any.
     */
    val scope: String?,

    /**
     * Properties associated with the trigger.
     */
    @Deprecated("Properties are being replaced by data; declare action arguments with @ActionData.")
    val properties: Map<String, NativeActionTriggerPropertyModel>,

    /**
     * Data associated with the trigger.
     */
    val data: Map<String, NativeActionTriggerDataModel>,

    /**
     * Events within the trigger that nested triggers can be filed under.
     */
    val events: Map<String, NativeActionTriggerEventModel>,

    /**
     * List of sub-triggers nested within this trigger.
     */
    val subTriggers: List<NativeActionTriggerModel>? = null,
)

/**
 * Represents an event within an action trigger for holding nested triggers.
 */
data class NativeActionTriggerEventModel(
    /**
     * Name of the event.
     */
    val event: String,

    /**
     * Scope this event hands to the triggers under it, null when it declares none.
     */
    val scope: String?
)

/**
 * Represents a property associated with an action trigger, including its key, value, and type.
 */
@Deprecated("Properties are being replaced by data; declare action arguments with @ActionData.")
data class NativeActionTriggerPropertyModel(
    /**
     * Key identifying the property.
     */
    val key: String,

    /**
     * Value of the property.
     */
    val value: String,

    /**
     * Type of the property (e.g., string, boolean).
     */
    val type: String
)

/**
 * Represents a piece of data associated with an action trigger, including its key, value, and type.
 */
data class NativeActionTriggerDataModel(
    /**
     * Key identifying the data entry.
     */
    val key: String,

    /**
     * Value of the data entry.
     */
    val value: String,

    /**
     * Type of the data entry (e.g., string, integer).
     */
    val type: String
)
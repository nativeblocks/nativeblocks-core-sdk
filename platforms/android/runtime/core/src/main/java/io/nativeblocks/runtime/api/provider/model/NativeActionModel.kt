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
     * Defines what happens after the trigger is executed.
     */
    val then: NativeActionTriggerThen,

    /**
     * Properties associated with the trigger.
     */
    val properties: Map<String, NativeActionTriggerPropertyModel>,

    /**
     * Data associated with the trigger.
     */
    val data: Map<String, NativeActionTriggerDataModel>,

    /**
     * List of sub-triggers nested within this trigger.
     */
    val subTriggers: List<NativeActionTriggerModel>? = null,
)

/**
 * Defines the possible outcomes of executing a trigger.
 */
enum class NativeActionTriggerThen(val then: String) {
    /** Trigger executed successfully. */
    SUCCESS("SUCCESS"),

    /** Trigger execution failed. */
    FAILURE("FAILURE"),

    /** Proceed to the next trigger. */
    NEXT("NEXT"),

    /** End the trigger sequence. */
    END("END");

    companion object {
        /**
         * Converts a string representation to a corresponding enum value.
         * @param then The string representation of the trigger outcome.
         * @return The matching enum value or END if no match is found.
         */
        fun fromThen(then: String): NativeActionTriggerThen {
            return when (then) {
                "SUCCESS" -> SUCCESS
                "FAILURE" -> FAILURE
                "NEXT" -> NEXT
                "END" -> END
                else -> END
            }
        }
    }
}

/**
 * Represents a property associated with an action trigger, including its key, value, and type.
 */
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
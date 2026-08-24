@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.api.provider.model

import androidx.compose.runtime.Immutable

/**
 * Represents a block within a native UI framework, including its properties,
 * data, visibility, position, and hierarchical structure.
 */
@Immutable
data class NativeBlockModel(
    /**
     * Unique identifier of the block.
     */
    val id: String,

    /**
     * Identifier of the parent block to establish hierarchy.
     */
    val parentId: String,

    /**
     * Key of the parent block to establish hierarchy.
     */
    val parentKey: String,

    /**
     * version of the block.
     */
    val version: Int,

    /**
     * Slot name where the block resides.
     */
    val slot: String,

    /**
     * Type of the key used to identify the block.
     */
    val keyType: String,

    /**
     * Key associated with the block.
     */
    val key: String,

    /**
     * Scope this block requires from the slot it sits in, null when it accepts any.
     */
    val scope: String?,

    /**
     * Visibility state of the block (e.g., visible, hidden).
     */
    val visibility: String,

    /**
     * Position of the block in its container.
     */
    val position: Int,

    /**
     * Data associated with the block, mapped by keys to their values and types.
     */
    val data: Map<String, NativeBlockDataModel>,

    /**
     * Properties associated with the block, including values for different device types.
     */
    @Deprecated("Properties are being replaced by data; declare block arguments with @NativeBlockData.")
    val properties: Map<String, NativeBlockPropertyModel>,

    /**
     * Slots within the block for additional content or nested structures.
     */
    val slots: Map<String, NativeBlockSlotModel>,

    /**
     * Modifiers applied to the block, in the order they are applied.
     */
    val modifiers: List<NativeBlockModifierModel>,

    /**
     * Sub-blocks nested within this block, mapped by their identifiers.
     */
    val subBlocks: Map<String, List<String>>? = null,
)

/**
 * Represents a property of a native block, including device-specific values and its type.
 */
@Deprecated("Properties are being replaced by data; declare block arguments with @NativeBlockData.")
@Immutable
data class NativeBlockPropertyModel(
    /**
     * Key identifying the property.
     */
    val key: String,

    /**
     * Value of the property for mobile devices.
     */
    val valueMobile: String,

    /**
     * Value of the property for tablets.
     */
    val valueTablet: String,

    /**
     * Value of the property for desktop devices.
     */
    val valueDesktop: String,

    /**
     * Type of the property (e.g., string, boolean).
     */
    val type: String
)

/**
 * Represents a piece of data associated with a native block, including its key, value, and type.
 */
@Immutable
data class NativeBlockDataModel(
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

/**
 * Represents a slot within a native block for holding additional content.
 */
@Immutable
data class NativeBlockSlotModel(
    /**
     * Name of the slot.
     */
    val slot: String,

    /**
     * Scope this slot hands to the blocks inside it, null when it declares none.
     */
    val scope: String?
)
/**
 * Represents a modifier attached to a block, decorating it without rendering content of its own.
 */
@Immutable
data class NativeBlockModifierModel(
    /**
     * Type of the key used to identify the modifier.
     */
    val keyType: String,

    /**
     * Order the modifier is applied in; lower runs first.
     */
    val position: Int,

    /**
     * Scope this modifier requires from the slot its host block sits in, null when it accepts any.
     */
    val scope: String?,

    /**
     * Data associated with the modifier.
     */
    val data: Map<String, NativeBlockModifierDataModel>,
)

/**
 * Represents a piece of data associated with a block modifier, including its key, value, and type.
 */
@Immutable
data class NativeBlockModifierDataModel(
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

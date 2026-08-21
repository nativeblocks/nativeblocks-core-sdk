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
    val slot: String
)
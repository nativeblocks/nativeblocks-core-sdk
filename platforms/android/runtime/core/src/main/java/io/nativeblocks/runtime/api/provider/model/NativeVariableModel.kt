package io.nativeblocks.runtime.api.provider.model

import androidx.compose.runtime.Immutable

/**
 * Represents a variable in Nativeblocks, including its key, value, and type.
 */
@Immutable
data class NativeVariableModel(
    /**
     * Key identifying the variable.
     */
    val key: String,

    /**
     * Value assigned to the variable.
     */
    val value: String,

    /**
     * Type of the variable (e.g., STRING, BOOLEAN, INT, DOUBLE, LONG, FLOAT).
     */
    val type: String
)
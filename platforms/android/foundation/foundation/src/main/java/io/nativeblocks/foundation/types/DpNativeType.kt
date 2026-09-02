package io.nativeblocks.foundation.types

import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [Dp] objects.
 * It implements the [INativeType] interface for [Dp].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Dp::class, DpNativeType())
 * ```
 */
class DpNativeType : INativeType<Dp> {

    /**
     * Converts a [Dp] object to its string representation.
     *
     * @param input The [Dp] object to be converted to a string.
     * @return A string representation of the [Dp] value, or "0.0" if the input is null.
     */
    override fun toString(input: Dp?): String {
        return input?.value?.toDouble()?.toString() ?: "0.0"
    }

    /**
     * Converts a string to a [Dp] object.
     *
     * @param input The string representation of a [Dp] value.
     * @return The corresponding [Dp] object, or 0.dp if the input is null or not a valid number.
     */
    override fun fromString(input: String?): Dp {
        return input?.toDoubleOrNull()?.dp ?: 0.dp
    }
}
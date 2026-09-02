package io.nativeblocks.foundation.types

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.ui.unit.dp
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [Arrangement.Horizontal] objects.
 * It implements the [INativeType] interface for [Arrangement.Horizontal].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Arrangement.Horizontal::class, HorizontalArrangementNativeType())
 * ```
 */
class HorizontalArrangementNativeType : INativeType<Arrangement.Horizontal> {

    /**
     * Converts an [Arrangement.Horizontal] object to its string representation.
     *
     * @param input The [Arrangement.Horizontal] object to be converted to a string.
     * @return A string representation of the [Arrangement.Horizontal], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: Arrangement.Horizontal?): String {
        return when (input) {
            Arrangement.Start -> "start"
            Arrangement.End -> "end"
            Arrangement.Center -> "center"
            Arrangement.SpaceBetween -> "spaceBetween"
            Arrangement.SpaceAround -> "spaceAround"
            Arrangement.SpaceEvenly -> "spaceEvenly"
            else -> ""
        }
    }

    /**
     * Converts a string to an [Arrangement.Horizontal] object.
     *
     * @param input The string representation of an [Arrangement.Horizontal].
     * @return The corresponding [Arrangement.Horizontal] object, or [Arrangement.Start] if the input is null or not recognized.
     */
    override fun fromString(input: String?): Arrangement.Horizontal {
        return when (input) {
            "start" -> Arrangement.Start
            "end" -> Arrangement.End
            "center" -> Arrangement.Center
            "spaceBetween" -> Arrangement.SpaceBetween
            "spaceAround" -> Arrangement.SpaceAround
            "spaceEvenly" -> Arrangement.SpaceEvenly
            else -> Arrangement.spacedBy(input?.toIntOrNull()?.dp ?: 0.dp)
        }
    }
}
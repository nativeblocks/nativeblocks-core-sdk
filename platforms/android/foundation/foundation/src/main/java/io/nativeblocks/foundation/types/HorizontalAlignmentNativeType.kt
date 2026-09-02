package io.nativeblocks.foundation.types

import androidx.compose.ui.Alignment
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [Alignment.Horizontal] objects.
 * It implements the [INativeType] interface for [Alignment.Horizontal].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Alignment.Horizontal::class, HorizontalAlignmentNativeType())
 * ```
 */
class HorizontalAlignmentNativeType : INativeType<Alignment.Horizontal> {

    /**
     * Converts an [Alignment.Horizontal] object to its string representation.
     *
     * @param input The [Alignment.Horizontal] object to be converted to a string.
     * @return A string representation of the [Alignment.Horizontal], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: Alignment.Horizontal?): String {
        return when (input) {
            Alignment.Start -> "start"
            Alignment.End -> "end"
            Alignment.CenterHorizontally -> "centerHorizontally"
            else -> ""
        }
    }

    /**
     * Converts a string to an [Alignment.Horizontal] object.
     *
     * @param input The string representation of an [Alignment.Horizontal].
     * @return The corresponding [Alignment.Horizontal] object, or [Alignment.Start] if the input is null or not recognized.
     */
    override fun fromString(input: String?): Alignment.Horizontal {
        return when (input) {
            "start" -> Alignment.Start
            "end" -> Alignment.End
            "centerHorizontally" -> Alignment.CenterHorizontally
            else -> Alignment.Start
        }
    }
}
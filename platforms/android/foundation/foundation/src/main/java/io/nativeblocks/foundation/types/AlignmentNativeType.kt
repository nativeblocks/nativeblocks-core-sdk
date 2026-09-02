package io.nativeblocks.foundation.types

import androidx.compose.ui.Alignment
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [Alignment] objects.
 * It implements the [INativeType] interface for [Alignment].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Alignment::class, AlignmentNativeType())
 * ```
 */
class AlignmentNativeType : INativeType<Alignment> {

    /**
     * Converts an [Alignment] object to its string representation.
     *
     * @param input The [Alignment] object to be converted to a string.
     * @return A string representation of the [Alignment], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: Alignment?): String {
        return when (input) {
            Alignment.Center -> "center"
            Alignment.CenterStart -> "centerStart"
            Alignment.CenterEnd -> "centerEnd"
            Alignment.BottomCenter -> "bottomCenter"
            Alignment.BottomStart -> "bottomStart"
            Alignment.BottomEnd -> "bottomEnd"
            Alignment.TopStart -> "topStart"
            Alignment.TopEnd -> "topEnd"
            Alignment.TopCenter -> "topCenter"
            else -> ""
        }
    }

    /**
     * Converts a string to an [Alignment] object.
     *
     * @param input The string representation of an [Alignment].
     * @return The corresponding [Alignment] object, or [Alignment.Center] if the input is null or not recognized.
     */
    override fun fromString(input: String?): Alignment {
        return when (input) {
            "center" -> Alignment.Center
            "centerStart" -> Alignment.CenterStart
            "centerEnd" -> Alignment.CenterEnd
            "bottomCenter" -> Alignment.BottomCenter
            "bottomStart" -> Alignment.BottomStart
            "bottomEnd" -> Alignment.BottomEnd
            "topStart" -> Alignment.TopStart
            "topEnd" -> Alignment.TopEnd
            "topCenter" -> Alignment.TopCenter
            else -> Alignment.Center
        }
    }
}
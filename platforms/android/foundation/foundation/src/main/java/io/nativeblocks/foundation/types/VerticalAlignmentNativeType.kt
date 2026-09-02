package io.nativeblocks.foundation.types

import androidx.compose.ui.Alignment
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that implements [INativeType] to handle conversion of [Alignment.Vertical] to and from strings.
 * This class is used to represent the [Alignment.Vertical] type in a string format, such as "top", "bottom", or "centerVertically".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Alignment.Vertical::class, VerticalAlignmentNativeType())
 * ```
 * In this example, the [VerticalAlignmentNativeType] is registered with the [NativeblocksManager] as a type converter
 * for the [Alignment.Vertical] class, enabling seamless conversion between [Alignment.Vertical] and its string representation.
 */
class VerticalAlignmentNativeType : INativeType<Alignment.Vertical> {

    /**
     * Converts the given [Alignment.Vertical] to a string representation.
     *
     * @param input The [Alignment.Vertical] value to convert.
     * @return A string representation of the [Alignment.Vertical] value, such as "top", "bottom", or "centerVertically".
     */
    override fun toString(input: Alignment.Vertical?): String {
        return when (input) {
            Alignment.Top -> "top"
            Alignment.Bottom -> "bottom"
            Alignment.CenterVertically -> "centerVertically"
            else -> ""
        }
    }

    /**
     * Converts the given string to the corresponding [Alignment.Vertical].
     *
     * @param input The string to convert, which should be one of the values: "top", "bottom", or "centerVertically".
     * @return The corresponding [Alignment.Vertical] value, such as [Alignment.Top], [Alignment.Bottom], or [Alignment.CenterVertically].
     *         Defaults to [Alignment.Top] if the string is invalid.
     */
    override fun fromString(input: String?): Alignment.Vertical {
        return when (input) {
            "top" -> Alignment.Top
            "bottom" -> Alignment.Bottom
            "centerVertically" -> Alignment.CenterVertically
            else -> Alignment.Top
        }
    }
}
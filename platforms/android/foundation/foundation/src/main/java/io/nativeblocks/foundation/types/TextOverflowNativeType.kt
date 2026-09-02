package io.nativeblocks.foundation.types

import androidx.compose.ui.text.style.TextOverflow
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that implements [INativeType] to handle [TextOverflow] conversion to and from strings.
 * This class is used to represent the [TextOverflow] type in a string format, such as "clip", "ellipsis", or "visible".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(TextOverflow::class, TextOverflowNativeType())
 * ```
 * In this example, the [TextOverflowNativeType] is registered with the [NativeblocksManager] as a type converter
 * for the [TextOverflow] class, enabling seamless conversion between [TextOverflow] and its string representation.
 */
class TextOverflowNativeType : INativeType<TextOverflow> {

    /**
     * Converts the given [TextOverflow] to a string representation.
     *
     * @param input The [TextOverflow] value to convert.
     * @return A string representation of the [TextOverflow] value: "clip", "ellipsis", or "visible".
     */
    override fun toString(input: TextOverflow?): String {
        return when (input) {
            TextOverflow.Clip -> "clip"
            TextOverflow.Ellipsis -> "ellipsis"
            TextOverflow.Visible -> "visible"
            else -> ""
        }
    }

    /**
     * Converts the given string to the corresponding [TextOverflow].
     *
     * @param input The string to convert, which should be one of the values: "clip", "ellipsis", or "visible".
     * @return The corresponding [TextOverflow] value, such as [TextOverflow.Clip], [TextOverflow.Ellipsis], or [TextOverflow.Visible].
     */
    override fun fromString(input: String?): TextOverflow {
        return when (input) {
            "clip" -> TextOverflow.Clip
            "ellipsis" -> TextOverflow.Ellipsis
            "visible" -> TextOverflow.Visible
            else -> TextOverflow.Clip
        }

    }
}
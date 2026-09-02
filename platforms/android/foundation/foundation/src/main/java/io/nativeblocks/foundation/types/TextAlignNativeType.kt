package io.nativeblocks.foundation.types

import androidx.compose.ui.text.style.TextAlign
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that implements [INativeType] to handle [TextAlign] conversion to and from strings.
 * This class is used to represent the [TextAlign] type in a string format, such as "start", "center", "end", or "justify".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(TextAlign::class, TextAlignNativeType())
 * ```
 * In this example, the [TextAlignNativeType] is registered with the [NativeblocksManager] as a type converter
 * for the [TextAlign] class, enabling seamless conversion between [TextAlign] and its string representation.
 */
class TextAlignNativeType : INativeType<TextAlign> {

    /**
     * Converts the given [TextAlign] to a string representation.
     *
     * @param input The [TextAlign] value to convert.
     * @return A string representation of the [TextAlign] value: "start", "center", "end", or "justify".
     */
    override fun toString(input: TextAlign?): String {
        return when (input) {
            TextAlign.Start -> "start"
            TextAlign.Center -> "center"
            TextAlign.End -> "end"
            TextAlign.Justify -> "justify"
            else -> "unspecified"
        }
    }

    /**
     * Converts the given string to the corresponding [TextAlign].
     *
     * @param input The string to convert, which should be one of the values: "start", "center", "end", or "justify".
     * @return The corresponding [TextAlign] value, such as [TextAlign.Start], [TextAlign.Center], [TextAlign.End], or [TextAlign.Justify].
     */
    override fun fromString(input: String?): TextAlign {
        return when (input) {
            "start" -> TextAlign.Start
            "center" -> TextAlign.Center
            "end" -> TextAlign.End
            "justify" -> TextAlign.Justify
            else -> TextAlign.Unspecified
        }
    }
}
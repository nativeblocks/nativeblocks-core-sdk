package io.nativeblocks.foundation.types

import androidx.compose.ui.text.font.FontWeight
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [FontWeight] objects.
 * It implements the [INativeType] interface for [FontWeight].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(FontWeight::class, FontWeightNativeType())
 * ```
 */
class FontWeightNativeType : INativeType<FontWeight> {

    /**
     * Converts a [FontWeight] object to its string representation.
     *
     * @param input The [FontWeight] object to be converted to a string.
     * @return A string representation of the [FontWeight], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: FontWeight?): String {
        return when (input) {
            FontWeight.Thin -> "thin"
            FontWeight.ExtraLight -> "extraLight"
            FontWeight.Light -> "light"
            FontWeight.Normal -> "normal"
            FontWeight.Medium -> "medium"
            FontWeight.SemiBold -> "semiBold"
            FontWeight.Bold -> "bold"
            FontWeight.ExtraBold -> "extraBold"
            FontWeight.Black -> "black"
            else -> ""
        }
    }

    /**
     * Converts a string to a [FontWeight] object.
     *
     * @param input The string representation of a [FontWeight].
     * @return The corresponding [FontWeight] object, or [FontWeight.Normal] if the input is null or not recognized.
     */
    override fun fromString(input: String?): FontWeight {
        return when (input) {
            "thin" -> FontWeight.Thin
            "extraLight" -> FontWeight.ExtraLight
            "light" -> FontWeight.Light
            "normal" -> FontWeight.Normal
            "medium" -> FontWeight.Medium
            "semiBold" -> FontWeight.SemiBold
            "bold" -> FontWeight.Bold
            "extraBold" -> FontWeight.ExtraBold
            "black" -> FontWeight.Black
            else -> FontWeight.Normal
        }
    }
}
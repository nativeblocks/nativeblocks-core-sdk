package io.nativeblocks.foundation.types

import androidx.compose.ui.graphics.Color
import androidx.core.graphics.toColorInt
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [Color] objects.
 * It implements the [INativeType] interface for [Color].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Color::class, ColorNativeType())
 * ```
 */
class ColorNativeType : INativeType<Color> {

    /**
     * Converts a [Color] object to its string representation in ARGB format.
     *
     * @param input The [Color] object to be converted to a string.
     * @return A string representation of the [Color] in ARGB format (e.g., "#AARRGGBB"), or "#00000000" if the input is null or an error occurs.
     */
    override fun toString(input: Color?): String {
        return try {
            input?.let {
                val alpha = (it.alpha * 255).toInt()
                val red = (it.red * 255).toInt()
                val green = (it.green * 255).toInt()
                val blue = (it.blue * 255).toInt()
                String.format("#%02X%02X%02X%02X", alpha, red, green, blue)
            } ?: "#00000000"
        } catch (e: Exception) {
            "#00000000"
        }
    }

    /**
     * Converts a string in ARGB format to a [Color] object.
     *
     * @param input The string representation of a [Color] in ARGB format (e.g., "#AARRGGBB").
     * @return The corresponding [Color] object, or [Color.Transparent] if the input is null, invalid, or an error occurs.
     */
    override fun fromString(input: String?): Color {
        return try {
            input?.toColorInt()?.let { Color(it) } ?: Color.Transparent
        } catch (e: Exception) {
            Color.Transparent
        }
    }
}
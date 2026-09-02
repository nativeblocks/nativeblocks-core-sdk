package io.nativeblocks.foundation.types

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.ui.unit.dp
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that implements [INativeType] to handle conversion of [Arrangement.Vertical] to and from strings.
 * This class is used to represent the [Arrangement.Vertical] type in a string format, such as "top", "bottom", "center",
 * "spaceBetween", "spaceAround", or "spaceEvenly".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(Arrangement.Vertical::class, VerticalArrangementNativeType())
 * ```
 * In this example, the [VerticalArrangementNativeType] is registered with the [NativeblocksManager] as a type converter
 * for the [Arrangement.Vertical] class, enabling seamless conversion between [Arrangement.Vertical] and its string representation.
 */
class VerticalArrangementNativeType : INativeType<Arrangement.Vertical> {

    /**
     * Converts the given [Arrangement.Vertical] to a string representation.
     *
     * @param input The [Arrangement.Vertical] value to convert.
     * @return A string representation of the [Arrangement.Vertical] value, such as "top", "bottom", "center",
     *         "spaceBetween", "spaceAround", or "spaceEvenly".
     */
    override fun toString(input: Arrangement.Vertical?): String {
        return when (input) {
            Arrangement.Top -> "top"
            Arrangement.Bottom -> "bottom"
            Arrangement.Center -> "center"
            Arrangement.SpaceBetween -> "spaceBetween"
            Arrangement.SpaceAround -> "spaceAround"
            Arrangement.SpaceEvenly -> "spaceEvenly"
            else -> ""
        }
    }

    /**
     * Converts the given string to the corresponding [Arrangement.Vertical].
     *
     * @param input The string to convert, which should be one of the values: "top", "bottom", "center",
     *              "spaceBetween", "spaceAround", or "spaceEvenly".
     * @return The corresponding [Arrangement.Vertical] value, such as [Arrangement.Top], [Arrangement.Bottom],
     *         [Arrangement.Center], [Arrangement.SpaceBetween], [Arrangement.SpaceAround], or [Arrangement.SpaceEvenly].
     *         Defaults to [Arrangement.Top] if the string is invalid.
     */
    override fun fromString(input: String?): Arrangement.Vertical {
        return when (input) {
            "top" -> Arrangement.Top
            "bottom" -> Arrangement.Bottom
            "center" -> Arrangement.Center
            "spaceBetween" -> Arrangement.SpaceBetween
            "spaceAround" -> Arrangement.SpaceAround
            "spaceEvenly" -> Arrangement.SpaceEvenly
            else -> Arrangement.spacedBy(input?.toIntOrNull()?.dp ?: 0.dp)
        }
    }
}
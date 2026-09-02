package io.nativeblocks.foundation.types

import androidx.compose.ui.unit.LayoutDirection
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that implements [INativeType] to handle [LayoutDirection] conversion to and from strings.
 * This class is used to represent the [LayoutDirection] type in a string format, either "LTR" or "RTL".
 *
 * Example usage:
 * ```
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(LayoutDirection::class, LayoutDirectionNativeType())
 * ```
 * In this example, the [LayoutDirectionNativeType] is registered with the [NativeblocksManager] as a type converter
 * for the [LayoutDirection] class, enabling seamless conversion between [LayoutDirection] and its string representation.
 */
class LayoutDirectionNativeType : INativeType<LayoutDirection> {

    /**
     * Converts the given [LayoutDirection] to a string representation.
     *
     * @param input The [LayoutDirection] to convert.
     * @return A string representation of the [LayoutDirection], either "LTR" or "RTL".
     */
    override fun toString(input: LayoutDirection?): String {
        return if (input == LayoutDirection.Rtl) {
            "RTL"
        } else {
            "LTR"
        }
    }

    /**
     * Converts the given string to the corresponding [LayoutDirection].
     *
     * @param input The string to convert, which should be either "RTL" or "LTR".
     * @return The corresponding [LayoutDirection], either [LayoutDirection.Rtl] or [LayoutDirection.Ltr].
     */
    override fun fromString(input: String?): LayoutDirection {
        return if (input?.uppercase() == "RTL") {
            LayoutDirection.Rtl
        } else {
            LayoutDirection.Ltr
        }
    }
}
package io.nativeblocks.foundation.types

import androidx.compose.ui.text.input.KeyboardType
import io.nativeblocks.runtime.api.provider.type.INativeType

/**
 * A class that provides serialization and deserialization methods for [KeyboardType] objects.
 * It implements the [INativeType] interface for [KeyboardType].
 *
 * Example usage:
 * ```kotlin
 * NativeblocksManager.getInstance()
 *     .provideTypeConverter(KeyboardType::class, KeyboardTypeNativeType())
 * ```
 */
class KeyboardTypeNativeType : INativeType<KeyboardType> {

    /**
     * Converts a [KeyboardType] object to its string representation.
     *
     * @param input The [KeyboardType] object to be converted to a string.
     * @return A string representation of the [KeyboardType], or an empty string if the input is null or not recognized.
     */
    override fun toString(input: KeyboardType?): String {
        return when (input) {
            KeyboardType.Text -> "text"
            KeyboardType.Ascii -> "ascii"
            KeyboardType.Number -> "number"
            KeyboardType.Phone -> "phone"
            KeyboardType.Uri -> "uri"
            KeyboardType.Email -> "email"
            KeyboardType.Password -> "password"
            KeyboardType.NumberPassword -> "numberPassword"
            KeyboardType.Decimal -> "decimal"
            else -> ""
        }
    }

    /**
     * Converts a string to a [KeyboardType] object.
     *
     * @param input The string representation of a [KeyboardType].
     * @return The corresponding [KeyboardType] object, or [KeyboardType.Text] if the input is null or not recognized.
     */
    override fun fromString(input: String?): KeyboardType {
        return when (input) {
            "text" -> KeyboardType.Text
            "ascii" -> KeyboardType.Ascii
            "number" -> KeyboardType.Number
            "phone" -> KeyboardType.Phone
            "uri" -> KeyboardType.Uri
            "email" -> KeyboardType.Email
            "password" -> KeyboardType.Password
            "numberPassword" -> KeyboardType.NumberPassword
            "decimal" -> KeyboardType.Decimal
            else -> KeyboardType.Unspecified
        }
    }
}
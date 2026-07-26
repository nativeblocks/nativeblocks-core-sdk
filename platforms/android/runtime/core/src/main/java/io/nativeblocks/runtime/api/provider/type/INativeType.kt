package io.nativeblocks.runtime.api.provider.type

/**
 * Interface for converting a type to and from its string representation.
 *
 * @param T The type to be converted.
 */
interface INativeType<T> {

    /**
     * Converts an object of type `T` to its string representation.
     *
     * @param input The object to be converted.
     * @return The string representation of the input object.
     */
    fun toString(input: T?): String

    /**
     * Converts a string back to an object of type `T`.
     *
     * @param input The string to be converted.
     * @return The object representation of the string.
     */
    fun fromString(input: String?): T
}
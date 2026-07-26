package io.nativeblocks.runtime.api.provider.type

import java.util.concurrent.ConcurrentHashMap
import kotlin.reflect.KClass


/**
 * A provider for handling type converters for various data types.
 * This class maps specific types to their respective converters,
 * allowing easy conversion between objects and their string representations.
 */
internal class NativeTypeProvider {

    private val types = mutableMapOf<KClass<*>, INativeType<*>>()

    /**
     * Registers a type converter for a given type.
     *
     * @param T The type for which the converter is being provided.
     * @param type The `KClass` of the type.
     * @param converter The implementation of `INativeType` to handle conversion for the type.
     */
    fun <T : Any> provideTypeConverter(type: KClass<T>, converter: INativeType<T>) {
        types[type] = MemoizedNativeType(converter)
    }

    /**
     * Retrieves the type converter for a given type.
     *
     * @param T The type for which the converter is being retrieved.
     * @param type The `KClass` of the type.
     * @return The `INativeType` implementation for the requested type.
     * @throws NullPointerException If no converter has been registered for the requested type.
     */
    fun <T : Any> getTypeConverter(type: KClass<T>): INativeType<T> {
        try {
            @Suppress("UNCHECKED_CAST")
            return types[type] as INativeType<T>
        } catch (e: Exception) {
            throw NullPointerException("The ${type.qualifiedName} converter not provided.")
        }
    }
}

private class MemoizedNativeType<T : Any>(private val delegate: INativeType<T>) : INativeType<T> {

    private val cache = ConcurrentHashMap<String, T>()

    override fun toString(input: T?): String = delegate.toString(input)

    override fun fromString(input: String?): T {
        input ?: return delegate.fromString(null)
        return cache.getOrPut(input) { delegate.fromString(input) }
    }
}
package io.nativeblocks.devkit.lib.cache

import kotlinx.coroutines.flow.Flow

internal interface INativeCache {
    fun push(key: String, value: String?)
    fun push(key: String, value: Boolean)
    fun push(key: String, value: Float)
    fun push(key: String, value: Double)
    fun push(key: String, value: Int)
    fun push(key: String, value: Long)

    fun pull(key: String, defaultValue: String): String
    fun pull(key: String, defaultValue: Boolean): Boolean
    fun pull(key: String, defaultValue: Float): Float
    fun pull(key: String, defaultValue: Double): Double
    fun pull(key: String, defaultValue: Int): Int
    fun pull(key: String, defaultValue: Long): Long

    fun clear()

    fun delete(key: String)

    fun onValueChange(key: String, defaultValue: String): Flow<String>
    fun onValueChange(key: String, defaultValue: Boolean): Flow<Boolean>
    fun onValueChange(key: String, defaultValue: Int): Flow<Int>
    fun onValueChange(key: String, defaultValue: Long): Flow<Long>
    fun onValueChange(key: String, defaultValue: Float): Flow<Float>
    fun onValueChange(key: String, defaultValue: Double): Flow<Double>
}
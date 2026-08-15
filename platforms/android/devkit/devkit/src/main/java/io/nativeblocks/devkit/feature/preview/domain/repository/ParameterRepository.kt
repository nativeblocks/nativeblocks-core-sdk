package io.nativeblocks.devkit.feature.preview.domain.repository

import io.nativeblocks.devkit.feature.preview.domain.model.Parameter

internal interface ParameterRepository {
    fun getAll(): List<Parameter>
    fun add(key: String, value: String): Parameter
    fun update(id: String, key: String, value: String)
    fun delete(id: String)
    fun clear()
    fun toMap(): Map<String, String>
}

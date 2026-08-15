package io.nativeblocks.devkit.feature.preview.data

import io.nativeblocks.devkit.feature.preview.domain.model.Parameter
import io.nativeblocks.devkit.feature.preview.domain.repository.ParameterRepository

internal class ParameterRepositoryImpl : ParameterRepository {
    private val parameters = mutableListOf<Parameter>()

    override fun getAll(): List<Parameter> = parameters.toList()

    override fun add(key: String, value: String): Parameter {
        val param = Parameter(key = key, value = value)
        parameters.add(param)
        return param
    }

    override fun update(id: String, key: String, value: String) {
        val index = parameters.indexOfFirst { it.id == id }
        if (index >= 0) {
            parameters[index] = parameters[index].copy(key = key, value = value)
        }
    }

    override fun delete(id: String) {
        parameters.removeAll { it.id == id }
    }

    override fun clear() {
        parameters.clear()
    }

    override fun toMap(): Map<String, String> {
        return parameters.associate { it.key to it.value }
    }
}

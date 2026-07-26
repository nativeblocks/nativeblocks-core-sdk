package io.nativeblocks.runtime.experiment

import io.nativeblocks.runtime.ffi.NativeRuntimeClientManager

internal class ExperimentUseCase(
    private val runtimeClient: NativeRuntimeClientManager,
) {

    suspend fun get(key: String, cacheTTL: Long): Pair<String, String>? =
        runCatching {
            val result = runtimeClient.experimentClient.getExperiment(key, cacheTTL)
            result.value to result.variableType
        }.getOrNull()
}

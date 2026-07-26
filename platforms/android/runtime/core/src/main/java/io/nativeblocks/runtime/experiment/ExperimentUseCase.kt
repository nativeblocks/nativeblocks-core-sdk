package io.nativeblocks.runtime.experiment

import io.nativeblocks.runtime.engine.NativeEngineClientManager

internal class ExperimentUseCase(
    private val engineClient: NativeEngineClientManager,
) {

    suspend fun get(key: String, cacheTTL: Long): Pair<String, String>? =
        runCatching {
            val result = engineClient.experimentClient.getExperiment(key, cacheTTL)
            result.value to result.variableType
        }.getOrNull()
}

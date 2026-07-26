package io.nativeblocks.runtime.engine

import io.nativeblocks.runtime.BuildConfig
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.util.SDKConfig

internal class NativeEngineClientManager(
    private val instanceName: String,
    edition: NativeblocksEdition,
    http: HttpClient,
    private val cache: CacheProvider,
) {
    val environment = edition.toEngineEnvironment(instanceName)

    private val config = SdkConfig(
        version = BuildConfig.VERSION,
        platform = SDKConfig.SDK_PLATFORM,
    )

    private val engine: NativeblocksEngine = NativeblocksEngine(environment, config, http, cache)

    val frameClient: FrameClient = engine.frameClient()
    val scaffoldClient: ScaffoldClient = engine.scaffoldClient()
    val experimentClient: ExperimentClient = engine.experimentClient()
    val localizationClient: LocalizationClient = engine.localizationClient()
    val globalParameterClient: GlobalParameterClient = engine.globalParameterClient()

    fun frameStateManager(): FrameStateManager = frameClient.stateManager()
    fun localizationStateManager(): LocalizationStateManager = localizationClient.stateManager()

    fun warmup() {
        runCatching { cache.has("nativeblocks_warmup") }
    }

    fun close() {
        runCatching { frameClient.close() }
        runCatching { scaffoldClient.close() }
        runCatching { experimentClient.close() }
        runCatching { localizationClient.close() }
        runCatching { globalParameterClient.close() }
        runCatching { engine.close() }
        runCatching { disposeInstance(instanceName) }
        runCatching { cache.dispose() }
    }
}

private fun NativeblocksEdition.toEngineEnvironment(instanceName: String): NativeblocksEnvironment {
    return when (this) {
        is NativeblocksEdition.Cloud -> NativeblocksEnvironment(
            instanceName = instanceName,
            endpoint = endpoint,
            apiKey = apiKey,
            developmentMode = developmentMode,
        )

        is NativeblocksEdition.Community -> NativeblocksEnvironment(
            instanceName = instanceName,
            endpoint = "",
            apiKey = "",
            developmentMode = false,
        )
    }
}

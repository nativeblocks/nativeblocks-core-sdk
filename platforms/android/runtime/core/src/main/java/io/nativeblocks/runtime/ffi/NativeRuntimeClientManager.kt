package io.nativeblocks.runtime.ffi

import io.nativeblocks.runtime.BuildConfig
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.util.SDKConfig
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

internal class NativeRuntimeClientManager(
    private val instanceName: String,
    edition: NativeblocksEdition,
    http: HttpClient,
    cacheDir: String,
) {
    val environment = edition.toEngineEnvironment(instanceName)

    private val config = SdkConfig(
        version = BuildConfig.VERSION,
        platform = SDKConfig.SDK_PLATFORM,
    )

    private val runtime: NativeblocksRuntime =
        NativeblocksRuntime(environment, config, http, cacheDir)

    val frameClient: FrameClient = runtime.frameClient()
    val scaffoldClient: ScaffoldClient = runtime.scaffoldClient()
    val experimentClient: ExperimentClient = runtime.experimentClient()
    val localizationClient: LocalizationClient = runtime.localizationClient()
    val globalParameterClient: GlobalParameterClient = runtime.globalParameterClient()

    fun frameStateManager(): FrameStateManager = frameClient.stateManager()
    fun localizationStateManager(): LocalizationStateManager = localizationClient.stateManager()

    fun close() {
        runCatching { frameClient.close() }
        runCatching { scaffoldClient.close() }
        runCatching { experimentClient.close() }
        runCatching { localizationClient.close() }
        runCatching { globalParameterClient.close() }
        runCatching { runtime.close() }
        runCatching { disposeInstance(instanceName) }
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

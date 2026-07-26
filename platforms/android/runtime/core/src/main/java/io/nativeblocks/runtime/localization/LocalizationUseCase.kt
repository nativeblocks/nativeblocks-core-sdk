package io.nativeblocks.runtime.localization

import io.nativeblocks.runtime.ffi.LocalizationStateManager
import io.nativeblocks.runtime.ffi.NativeRuntimeClientManager
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

internal class LocalizationUseCase(
    private val runtimeClient: NativeRuntimeClientManager,
    private val stateManager: LocalizationStateManager,
) {

    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val client get() = runtimeClient.localizationClient

    fun setLocalization(languageCode: String) {
        scope.launch {
            runCatching { client.setLanguageCode(languageCode) }
            runCatching { client.syncLocalization(languageCode) }
        }
    }

    fun translate(key: String): Result<String?> {
        return runCatching { stateManager.translate(key) }
    }
}

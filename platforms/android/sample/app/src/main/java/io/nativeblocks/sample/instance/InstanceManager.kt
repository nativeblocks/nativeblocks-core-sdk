package io.nativeblocks.sample.instance

import android.content.Context
import android.util.Log
import io.nativeblocks.foundation.FoundationProvider
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.NativeblocksManager
import io.nativeblocks.runtime.api.provider.logger.INativeLogger
import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel
import io.nativeblocks.sample.integration.consumer.block.SampleBlockProvider

class InstanceManager(private val applicationContext: Context) {

    val instances: List<InstanceInfo> get() = InstanceCatalog.instances
    fun start(instance: InstanceInfo): NativeblocksManager {
        val wasRunning = isRunning(instance)
        val manager = NativeblocksManager.initialize(
            name = instance.instanceKey,
            applicationContext = applicationContext,
            edition = NativeblocksEdition.Cloud(
                endpoint = instance.apiUrl,
                apiKey = instance.apiKey,
                developmentMode = instance.developmentMode,
            ),
        )
        if (!wasRunning) {
            SampleBlockProvider.provideBlocks(instanceName = instance.instanceKey)
            FoundationProvider.provide(instanceName = instance.instanceKey)
            manager.provideEventLogger("LOGGER", Logger())
        }
        return manager
    }

    fun isRunning(instance: InstanceInfo): Boolean =
        NativeblocksManager.isInitialized(instance.instanceKey)

    fun stop(instance: InstanceInfo) {
        if (isRunning(instance)) {
            NativeblocksManager.getInstance(instance.instanceKey).destroy()
        }
    }

    fun stopAll(): Unit = instances.forEach(::stop)

}

internal class Logger : INativeLogger {

    override fun log(level: LoggerEventLevel, event: String, message: String, parameters: Map<String, String>) {
        val paramsText = if (parameters.isNotEmpty()) {
            parameters.entries.joinToString("\n") { "║   ${it.key}: ${it.value}" }
        } else {
            "║   (none)"
        }
        val logString = """
            |╔═══════════════════════════════════════════════════════════
            |║ Nativeblocks Log
            |╠═══════════════════════════════════════════════════════════
            |║ Level   : ${level.name}
            |║ Event   : $event
            |║ Message : 
            |║   $message
            |║ Parameters:
            |$paramsText
            |╚═══════════════════════════════════════════════════════════
        """.trimMargin()

        Log.d("Nativeblocks", logString)
    }
}

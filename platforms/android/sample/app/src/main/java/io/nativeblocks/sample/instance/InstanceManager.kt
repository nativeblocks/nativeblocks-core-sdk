package io.nativeblocks.sample.instance

import android.content.Context
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.compose.ui.unit.Dp
import io.nativeblocks.devkit.DevKit
import io.nativeblocks.devkit.PreviewKit
import io.nativeblocks.foundation.FoundationProvider
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.NativeblocksManager
import io.nativeblocks.runtime.api.provider.logger.INativeLogger
import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel
import io.nativeblocks.runtime.api.provider.type.INativeType
import io.nativeblocks.sample.integration.consumer.block.AppBlockProvider
import io.nativeblocks.sample.integration.consumer.modifier.AppModifierProvider
import io.nativeblocks.sample.modifier.ShapeStyle
import io.nativeblocks.sample.modifier.ShapeStyleNativeType

class InstanceManager(private val applicationContext: Context) {

    val instances: List<InstanceInfo> get() = InstanceCatalog.instances
    fun start(instance: InstanceInfo, activity: ComponentActivity): NativeblocksManager {
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
            AppBlockProvider.provideBlocks(instanceName = instance.instanceKey)
            AppModifierProvider.provideModifiers(instanceName = instance.instanceKey)
            FoundationProvider.provide(instanceName = instance.instanceKey)
            manager.provideTypeConverter(ShapeStyle::class, ShapeStyleNativeType())
            manager.provideEventLogger("LOGGER", Logger())
            manager.provideKit(
                if (instance.developmentMode) {
                    DevKit.Builder(activity)
                        .keepScreenOn()
                        .autoConnect()
                        .logTracking()
                        .build()
                } else {
                    PreviewKit.Builder()
                        .launchOnShake()
                        .build()
                }
            )
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

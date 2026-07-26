package io.nativeblocks.sample.instance

import android.content.Context
import io.nativeblocks.foundation.FoundationProvider
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.api.NativeblocksManager
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

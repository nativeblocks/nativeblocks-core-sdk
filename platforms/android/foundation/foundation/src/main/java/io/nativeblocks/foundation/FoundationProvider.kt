package io.nativeblocks.foundation

import io.nativeblocks.foundation.integration.consumer.block.FoundationBlockProvider
import io.nativeblocks.foundation.integration.consumer.modifier.FoundationModifierProvider

object FoundationProvider {
    fun provide(instanceName: String = "default") {
        FoundationTypeProvider.provideTypes(instanceName = instanceName)
        FoundationBlockProvider.provideBlocks(instanceName = instanceName)
        FoundationModifierProvider.provideModifiers(instanceName = instanceName)
    }
}
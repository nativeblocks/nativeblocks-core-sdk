package io.nativeblocks.gradleplugin

import org.gradle.api.provider.Property

abstract class NativeblocksExtension {
    abstract val enableEmbedding: Property<Boolean>

    init {
        enableEmbedding.convention(false)
    }
}

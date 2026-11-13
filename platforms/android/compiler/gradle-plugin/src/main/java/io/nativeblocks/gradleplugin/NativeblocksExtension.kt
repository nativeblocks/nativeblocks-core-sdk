package io.nativeblocks.gradleplugin

import org.gradle.api.provider.Property

abstract class NativeblocksExtension {
    abstract val enableEmbedding: Property<Boolean>
    abstract val embeddingInstance: Property<String>

    init {
        enableEmbedding.convention(false)
        embeddingInstance.convention("default")
    }
}

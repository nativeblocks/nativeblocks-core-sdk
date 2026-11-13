package io.nativeblocks.runtime.lib

import android.content.res.AssetManager
import io.nativeblocks.runtime.ffi.EmbeddedReader

internal class AssetEmbeddedReader(private val assets: AssetManager) : EmbeddedReader {

    override fun read(path: String): ByteArray? {
        return runCatching {
            assets.open("$EMBEDDED_DIR/$path").use { it.readBytes() }
        }.getOrNull()
    }

    private companion object {
        const val EMBEDDED_DIR = "nativeblocksEmbedded"
    }
}

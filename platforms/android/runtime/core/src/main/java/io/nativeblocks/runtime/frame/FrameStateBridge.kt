package io.nativeblocks.runtime.frame

import io.nativeblocks.runtime.engine.FrameChangeType
import io.nativeblocks.runtime.engine.FrameDiff
import io.nativeblocks.runtime.engine.FrameFull
import io.nativeblocks.runtime.engine.FrameStateManager
import io.nativeblocks.runtime.engine.FrameStateObserver

internal interface FrameStateBridge {
    suspend fun observeFrame(
        route: String,
        args: Map<String, String>,
        onFull: (FrameFull) -> Unit,
        onDiff: (FrameDiff) -> Unit,
    )

    fun updateVariable(key: String, value: String)
    fun updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        valueMobile: String,
        valueTablet: String,
        valueDesktop: String
    )

    fun releaseFrame()
}

internal class FrameStateBridgeImpl(
    private val frameStateManager: FrameStateManager,
) : FrameStateBridge {

    override suspend fun observeFrame(
        route: String,
        args: Map<String, String>,
        onFull: (FrameFull) -> Unit,
        onDiff: (FrameDiff) -> Unit,
    ) {
        frameStateManager.setupFrame(route, args, object : FrameStateObserver {
            override fun onFrameChange(change: FrameChangeType) {
                when (change) {
                    is FrameChangeType.Full -> onFull(change.frame)
                    is FrameChangeType.Diff -> onDiff(change.frame)
                }
            }
        })
    }

    override fun updateVariable(key: String, value: String) {
        frameStateManager.updateVariable(key, value)
    }

    override fun updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        valueMobile: String,
        valueTablet: String,
        valueDesktop: String
    ) {
        frameStateManager.updateBlockProperty(
            blockKey,
            propertyKey,
            valueMobile,
            valueTablet,
            valueDesktop
        )
    }

    override fun releaseFrame() {
        frameStateManager.release()
    }
}

package io.nativeblocks.runtime.frame

import io.nativeblocks.runtime.ffi.ActionLogEvent
import io.nativeblocks.runtime.ffi.FrameChangeType
import io.nativeblocks.runtime.ffi.FrameDiff
import io.nativeblocks.runtime.ffi.FrameFull
import io.nativeblocks.runtime.ffi.FrameStateManager
import io.nativeblocks.runtime.ffi.FrameStateObserver

internal interface FrameStateBridge {
    suspend fun observeFrame(
        route: String,
        args: Map<String, String>,
        stateKey: String?,
        onFull: (FrameFull) -> Unit,
        onDiff: (FrameDiff) -> Unit,
    )

    fun updateVariable(key: String, value: String)

    fun updateBlockData(blockKey: String, dataKey: String, value: String)

    fun updateBlockProperty(
        blockKey: String,
        propertyKey: String,
        valueMobile: String,
        valueTablet: String,
        valueDesktop: String
    )

    fun logAction(event: ActionLogEvent)

    fun releaseFrame()
}

internal class FrameStateBridgeImpl(
    private val frameStateManager: FrameStateManager,
) : FrameStateBridge {

    override suspend fun observeFrame(
        route: String,
        args: Map<String, String>,
        stateKey: String?,
        onFull: (FrameFull) -> Unit,
        onDiff: (FrameDiff) -> Unit,
    ) {
        frameStateManager.setupFrame(route, args, stateKey, object : FrameStateObserver {
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

    override fun updateBlockData(blockKey: String, dataKey: String, value: String) {
        frameStateManager.updateBlockData(blockKey, dataKey, value)
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

    override fun logAction(event: ActionLogEvent) {
        frameStateManager.logAction(event)
    }

    override fun releaseFrame() {
        frameStateManager.release()
    }
}

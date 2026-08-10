package io.nativeblocks.runtime.api

sealed interface NativeblocksFrameState {
    object Stateless : NativeblocksFrameState
    data class Stateful(val key: String) : NativeblocksFrameState
}

internal val NativeblocksFrameState.key: String?
    get() = when (this) {
        is NativeblocksFrameState.Stateless -> null
        is NativeblocksFrameState.Stateful -> key
    }

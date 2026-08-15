package io.nativeblocks.devkit.feature.live.domain.repository

import io.nativeblocks.devkit.feature.live.domain.model.ConnectionState
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow

internal interface LiveRepository {
    suspend fun start()
    suspend fun end()
    suspend fun orientedChange()
    fun connectionState(): StateFlow<ConnectionState>
    fun hotReload(): SharedFlow<String>
}
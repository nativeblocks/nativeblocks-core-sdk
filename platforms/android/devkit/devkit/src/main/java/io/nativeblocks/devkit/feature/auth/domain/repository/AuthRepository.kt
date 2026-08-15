package io.nativeblocks.devkit.feature.auth.domain.repository

import kotlinx.coroutines.flow.Flow

internal interface AuthRepository {
    suspend fun updateTokenAndEndpoint(qrData: String): Boolean
    suspend fun updateTokenAndEndpoint(endpoint: String, token: String, realtimeEndpoint: String): Boolean
    suspend fun authCheck(): Boolean
    suspend fun logout()
    fun token(): Flow<String>
}
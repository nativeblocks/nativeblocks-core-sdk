package io.nativeblocks.devkit.feature.auth.data

import io.nativeblocks.devkit.feature.auth.domain.model.QrData
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.lib.cache.INativeCache
import io.nativeblocks.devkit.util.JWT
import io.nativeblocks.devkit.util.DevKitLogger
import kotlinx.coroutines.flow.Flow
import kotlinx.serialization.json.Json
import java.util.UUID

internal class AuthRepositoryImpl(private val cache: INativeCache) : AuthRepository {

    override suspend fun updateTokenAndEndpoint(qrData: String): Boolean {
        return try {
            val json = Json { ignoreUnknownKeys = true }
            val data = json.decodeFromString<QrData>(qrData)
            cache.push("ENDPOINT", data.endpoint)
            cache.push("REALTIME_ENDPOINT", data.realtimeEndpoint)

            val decodedBody = JWT.decodedBody(data.token)
            val username = try {
                decodedBody?.getString("email")
            } catch (e: Exception) {
                ""
            }
            cache.push("USERNAME", username)

            cache.push("UUID", UUID.randomUUID().toString())
            cache.push("AUTH_TOKEN", data.token)
            true
        } catch (e: Exception) {
            DevKitLogger.e("AuthRepository", "update", e)
            false
        }
    }

    override suspend fun updateTokenAndEndpoint(
        endpoint: String,
        token: String,
        realtimeEndpoint: String
    ): Boolean {
        return try {
            cache.push("ENDPOINT", endpoint)
            cache.push("REALTIME_ENDPOINT", realtimeEndpoint)

            val decodedBody = JWT.decodedBody(token)
            val username = try {
                decodedBody?.getString("email")
            } catch (e: Exception) {
                ""
            }
            cache.push("USERNAME", username)

            cache.push("UUID", UUID.randomUUID().toString())
            cache.push("AUTH_TOKEN", token)
            true
        } catch (e: Exception) {
            DevKitLogger.e("AuthRepository", "update", e)
            false
        }
    }

    override suspend fun authCheck(): Boolean {
        return cache.pull("AUTH_TOKEN", "").isEmpty().not()
    }

    override suspend fun logout() {
        cache.delete("ENDPOINT")
        cache.delete("AUTH_TOKEN")
        cache.delete("REALTIME_ENDPOINT")
        cache.delete("USERNAME")
        cache.delete("UUID")
    }

    override fun token(): Flow<String> {
        return cache.onValueChange("AUTH_TOKEN", "")
    }
}
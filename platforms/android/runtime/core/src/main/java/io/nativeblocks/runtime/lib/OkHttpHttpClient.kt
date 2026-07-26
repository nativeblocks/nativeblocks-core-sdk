package io.nativeblocks.runtime.lib

import io.nativeblocks.runtime.ffi.ErrorType
import io.nativeblocks.runtime.ffi.HttpClient
import io.nativeblocks.runtime.ffi.NbException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody

internal class OkHttpHttpClient(
    private val client: OkHttpClient
) : HttpClient {

    override suspend fun get(url: String, headers: Map<String, String>): String =
        execute(
            Request.Builder()
                .url(url)
                .applyHeaders(headers)
                .get()
                .build()
        )

    override suspend fun post(url: String, headers: Map<String, String>, body: String): String =
        execute(
            Request.Builder()
                .url(url)
                .applyHeaders(headers)
                .post(body.toRequestBody(JSON_MEDIA_TYPE))
                .build()
        )

    private suspend fun execute(request: Request): String = uniffiCallbackGuard(ErrorType.NETWORK) {
        withContext(Dispatchers.IO) {
            client.newCall(request).execute().use { response ->
                val payload = response.body?.string().orEmpty()
                if (!response.isSuccessful) {
                    throw NbException.Failure(
                        reason = "HTTP ${response.code} for ${request.url}: $payload",
                        errorType = ErrorType.NETWORK,
                        errorCode = response.code.toString(),
                    )
                }
                payload
            }
        }
    }

    private fun Request.Builder.applyHeaders(headers: Map<String, String>): Request.Builder {
        headers.forEach { (key, value) -> header(key, value) }
        return this
    }

    private companion object {
        val JSON_MEDIA_TYPE = "application/json; charset=utf-8".toMediaType()
    }
}

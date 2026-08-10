package io.nativeblocks.gradleplugin.network

import okhttp3.Interceptor
import okhttp3.Response

internal class AuthorizationInterceptor(
    private val authToken: String
) : Interceptor {

    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request().newBuilder()
            .addHeader("authorization", "Bearer $authToken")
            .build()

        return chain.proceed(request)
    }
}
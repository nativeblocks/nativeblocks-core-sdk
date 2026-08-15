package io.nativeblocks.devkit.session

import io.nativeblocks.devkit.lib.cache.INativeCache

internal class RemoteSessionImpl(private val cache: INativeCache) : IRemoteSession {
    override fun getEndpoint(): String {
        return cache.pull("ENDPOINT", "")
    }

    override fun getToken(): String {
        return cache.pull("AUTH_TOKEN", "")
    }

    override fun getRealtimeEndpoint(): String {
        return cache.pull("REALTIME_ENDPOINT", "")
    }

    override fun getUsername(): String {
        return cache.pull("USERNAME", "")
    }

    override fun getUUID(): String {
        return cache.pull("UUID", "")
    }
}
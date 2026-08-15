package io.nativeblocks.devkit.session

internal interface IRemoteSession {
    fun getEndpoint(): String
    fun getToken(): String
    fun getRealtimeEndpoint(): String
    fun getUsername(): String
    fun getUUID(): String
}
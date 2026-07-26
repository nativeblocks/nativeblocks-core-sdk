package io.nativeblocks.runtime.api.provider.action

internal object NativeActionProviderRegistry {

    private val providerMap = mutableMapOf<String, NativeActionProvider>()
    private val lock = Any()

    fun getOrCreate(instanceName: String): NativeActionProvider {
        synchronized(lock) {
            return providerMap.getOrPut(instanceName) {
                NativeActionProvider()
            }
        }
    }

    fun remove(instanceName: String) {
        synchronized(lock) {
            providerMap.remove(instanceName)
        }
    }

}

package io.nativeblocks.runtime.api.provider.block

internal object NativeBlockProviderRegistry {

    private val providerMap = mutableMapOf<String, NativeBlockProvider>()
    private val lock = Any()

    fun getOrCreate(instanceName: String): NativeBlockProvider {
        synchronized(lock) {
            return providerMap.getOrPut(instanceName) {
                NativeBlockProvider()
            }
        }
    }

    fun remove(instanceName: String) {
        synchronized(lock) {
            providerMap.remove(instanceName)
        }
    }

}

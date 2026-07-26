package io.nativeblocks.runtime.api.provider.type

internal object NativeTypeProviderRegistry {

    private val providerMap = mutableMapOf<String, NativeTypeProvider>()
    private val lock = Any()

    fun getOrCreate(instanceName: String): NativeTypeProvider {
        synchronized(lock) {
            return providerMap.getOrPut(instanceName) {
                NativeTypeProvider()
            }
        }
    }

    fun remove(instanceName: String) {
        synchronized(lock) {
            providerMap.remove(instanceName)
        }
    }

}
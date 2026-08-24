package io.nativeblocks.runtime.api.provider.modifier

internal object NativeModifierProviderRegistry {

    private val providerMap = mutableMapOf<String, NativeModifierProvider>()
    private val lock = Any()

    fun getOrCreate(instanceName: String): NativeModifierProvider {
        synchronized(lock) {
            return providerMap.getOrPut(instanceName) {
                NativeModifierProvider()
            }
        }
    }

    fun remove(instanceName: String) {
        synchronized(lock) {
            providerMap.remove(instanceName)
        }
    }

}

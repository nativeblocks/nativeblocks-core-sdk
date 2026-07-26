import Foundation

internal class NativeBlockProviderRegistry {
    private static var providerMap: [String: NativeBlockProvider] = [:]
    private static let lock = NSLock()

    static func getOrCreate(_ instanceName: String) -> NativeBlockProvider {
        lock.lock()
        defer { lock.unlock() }

        if let existingProvider = providerMap[instanceName] {
            return existingProvider
        } else {
            let newProvider = NativeBlockProvider()
            providerMap[instanceName] = newProvider
            return newProvider
        }
    }

    static func remove(_ instanceName: String) {
        lock.lock()
        defer { lock.unlock() }
        providerMap.removeValue(forKey: instanceName)
    }

}

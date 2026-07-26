import Foundation

internal class NativeTypeProviderRegistry {
    private static var providerMap: [String: NativeTypeProvider] = [:]
    private static let lock = NSLock()

    static func getOrCreate(_ instanceName: String) -> NativeTypeProvider {
        lock.lock()
        defer { lock.unlock() }

        if let existingProvider = providerMap[instanceName] {
            return existingProvider
        } else {
            let newProvider = NativeTypeProvider()
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

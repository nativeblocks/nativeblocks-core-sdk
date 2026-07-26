import Foundation

internal class NativeActionProviderRegistry {
    private static var providerMap: [String: NativeActionProvider] = [:]
    private static let lock = NSLock()

    static func getOrCreate(_ instanceName: String) -> NativeActionProvider {
        lock.lock()
        defer { lock.unlock() }

        if let existingProvider = providerMap[instanceName] {
            return existingProvider
        } else {
            let newProvider = NativeActionProvider()
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

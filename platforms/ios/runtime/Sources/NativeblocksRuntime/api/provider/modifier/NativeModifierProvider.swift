import Foundation
import SwiftUI

/// Provides and manages native modifiers within the framework.
internal class NativeModifierProvider {

    private var modifiers: [String: (AnyView, ModifierContext) -> AnyView] = [:]

    /// Registers a new modifier implementation for a specific modifier type.
    func provideModifier(modifierType: String, modifier: @escaping (AnyView, ModifierContext) -> AnyView) {
        modifiers[modifierType] = modifier
    }

    /// Retrieves all registered modifier implementations.
    func getProvidedModifiers() -> [String: (AnyView, ModifierContext) -> AnyView] {
        return modifiers
    }
}

internal class NativeModifierProviderRegistry {
    private static var providerMap: [String: NativeModifierProvider] = [:]
    private static let lock = NSLock()

    static func getOrCreate(_ instanceName: String) -> NativeModifierProvider {
        lock.lock()
        defer { lock.unlock() }
        if let provider = providerMap[instanceName] {
            return provider
        }
        let provider = NativeModifierProvider()
        providerMap[instanceName] = provider
        return provider
    }

    static func remove(_ instanceName: String) {
        lock.lock()
        defer { lock.unlock() }
        providerMap.removeValue(forKey: instanceName)
    }
}

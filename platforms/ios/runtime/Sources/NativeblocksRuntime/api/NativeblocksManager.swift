import Foundation
import NativeblocksRuntimeFFI
import SwiftUI

/// Manages the lifecycle and configuration of the Nativeblocks framework.
/// The `NativeblocksManager` is used to initialize the SDK, register blocks, actions and loggers,
/// and to reach the runtime for frames, localization and experiments.
public class NativeblocksManager {

    private static var instanceRegistry: [String: NativeblocksManager] = [:]
    private static let lock = NSLock()

    private let name: String
    private let edition: NativeblocksEdition
    private let typeProvider: NativeTypeProvider
    private var loggerTypes = Set<String>()
    private var actionContractors = [any INativeActionContractor]()
    private var kits = [any Kit]()

    internal func providedActionContractors() -> [any INativeActionContractor] {
        return actionContractors
    }

    /// Private initializer to enforce singleton usage.
    private init(name: String, edition: NativeblocksEdition) {
        self.name = name
        self.edition = edition
        self.typeProvider = NativeTypeProviderRegistry.getOrCreate(name)
        NativeCoreSDKInjector.initialize(name: name, edition: edition)
    }

    private var injector: NativeCoreSDKInjector {
        return NativeCoreSDKInjector.get(name: name)
    }

    private var blockProvider: NativeBlockProvider {
        return NativeBlockProviderRegistry.getOrCreate(name)
    }

    private var actionProvider: NativeActionProvider {
        return NativeActionProviderRegistry.getOrCreate(name)
    }

    private var modifierProvider: NativeModifierProvider {
        return NativeModifierProviderRegistry.getOrCreate(name)
    }

    /// Initializes the `NativeblocksManager` with the specified edition.
    /// - Parameters:
    ///   - name: The instance name. Valid characters: A–Z, a–z, 0–9, _ or -.
    ///   - edition: The `NativeblocksEdition` that determines the configuration type, such as cloud or community.
    @discardableResult
    public static func initialize(name: String = "default", edition: NativeblocksEdition) -> NativeblocksManager {
        lock.lock()
        defer { lock.unlock() }

        if !name.isValidInstanceName() {
            fatalError("Please make sure the '\(name)' contains valid characters: A–Z, a–z, 0–9, _ or -")
        }
        if let existing = instanceRegistry[name] {
            return existing
        }
        let manager = NativeblocksManager(name: name, edition: edition)
        instanceRegistry[name] = manager
        return manager
    }

    /// Check whether the instance has been initialized or not
    /// - Returns: Boolean.
    public static func isInitialized(name: String = "default") -> Bool {
        lock.lock()
        defer { lock.unlock() }
        return instanceRegistry[name] != nil
    }

    /// Gets the shared instance of the `NativeblocksManager`.
    /// - Returns: The shared `NativeblocksManager` instance.
    public static func getInstance(name: String = "default") -> NativeblocksManager {
        lock.lock()
        defer { lock.unlock() }
        guard let instance = instanceRegistry[name] else {
            fatalError("NativeblocksManager '\(name)' has not been initialized.")
        }
        return instance
    }

    /// Provides a block implementation.
    /// - Parameters:
    ///   - blockType: The type of the block.
    ///   - block: The view builder rendering the block.
    @discardableResult
    public func provideBlock(blockType: String, block: NativeBlock) -> NativeblocksManager {
        blockProvider.provideBlock(blockType: blockType, block: block)
        return self
    }

    /// Provides a modifier implementation.
    /// - Parameters:
    ///   - modifierType: The type of the modifier.
    ///   - modifier: A closure applying the modifier to the content it decorates.
    /// - Returns: The current NativeblocksManager instance.
    @discardableResult
    public func provideModifier(
        modifierType: String,
        modifier: @escaping (AnyView, ModifierContext) -> AnyView
    ) -> NativeblocksManager {
        modifierProvider.provideModifier(modifierType: modifierType, modifier: modifier)
        return self
    }

    /// Provides a fallback block to be used when a requested block keyType is not found.
    /// This block will be displayed in place of unsupported or unrecognized blocks.
    ///
    /// - Parameter block: A view builder receiving the missing block's keyType and key.
    @discardableResult
    public func provideFallbackBlock(block: @escaping (String, String) -> any View) -> NativeblocksManager {
        blockProvider.onFallbackBlock(block: block)
        return self
    }

    /// Provides an action implementation.
    /// - Parameters:
    ///   - actionType: The type of the action.
    ///   - action: The action instance implementing the `INativeAction` protocol.
    @discardableResult
    public func provideAction(actionType: String, action: any INativeAction) -> NativeblocksManager {
        actionProvider.provideAction(actionType: actionType, action: action)
        return self
    }

    /// Provides an action contractor.
    /// - Parameter actionContractor: The action contractor to provide.
    @discardableResult
    public func provideActionContractor(_ actionContractor: any INativeActionContractor) -> NativeblocksManager {
        actionContractors.append(actionContractor)
        actionProvider.provideActionContractor(actionContractor)
        return self
    }

    /// Provides a fallback action to be used when a requested action keyType is not found.
    /// This action will be executed in place of unsupported or unrecognized actions.
    ///
    /// - Parameter block: A closure receiving the missing action's keyType and name.
    @discardableResult
    public func provideFallbackAction(block: @escaping (String, String) -> Void) -> NativeblocksManager {
        actionProvider.onFallbackAction(block: block)
        return self
    }

    /// Provides an event logger for the specified logger type.
    /// The logger is registered with the runtime, so it receives every event the engine emits.
    /// - Parameters:
    ///   - loggerType: The key type associated with the logger.
    ///   - logger: The logger instance implementing the `INativeLogger` protocol.
    @discardableResult
    public func provideEventLogger(loggerType: String, logger: any INativeLogger) -> NativeblocksManager {
        provideLogger(instanceName: name, loggerType: loggerType, logger: LoggerAdapter(delegate: logger))
        loggerTypes.insert(loggerType)
        return self
    }

    /// Provides a kit.
    /// - Parameter kit: The kit to provide.
    @discardableResult
    public func provideKit(_ kit: any Kit) -> NativeblocksManager {
        kit.attach(instanceName: name, edition: edition)
        kits.append(kit)
        return self
    }

    /// Registers a type converter for a specific type.
    /// - Parameters:
    ///   - type: The type for which the converter is being registered.
    ///   - converter: The converter that implements `INativeType` for the given type.
    @discardableResult
    public func provideTypeConverter<T>(_ type: T.Type, converter: INativeType<T>) -> NativeblocksManager {
        typeProvider.provideTypeConverter(type, converter: converter)
        return self
    }

    /// Retrieves the type converter for a specific type.
    /// - Parameter type: The type for which the converter is being retrieved.
    /// - Returns: The converter that implements `INativeType` for the given type.
    public func getTypeConverter<T>(_ type: T.Type) -> INativeType<T> {
        return typeProvider.getTypeConverter(type)
    }

    /// Synchronizes the frame for the specified route asynchronously.
    /// - Parameter route: The route of the frame to synchronize.
    @discardableResult
    public func syncFrame(route: String) async -> NativeblocksManager {
        try? await injector.runtimeClient.frameClient.syncFrame(route: route, parameters: [:])
        return self
    }

    /// Clear all frames from the cache.
    /// - Returns: The `NativeblocksManager` instance for chaining.
    @discardableResult
    public func clearAllFrames() async -> NativeblocksManager {
        try? await injector.runtimeClient.frameClient.clearAll(routes: [])
        return self
    }

    /// Clear a specific frame from the cache.
    /// - Parameter route: The route of the frame to clear.
    /// - Returns: The `NativeblocksManager` instance for chaining.
    @discardableResult
    public func clearFrame(route: String) async -> NativeblocksManager {
        try? await injector.runtimeClient.frameClient.clear(route: route)
        return self
    }

    /// Throws away the frame kept under `key`, so the next visit starts fresh.
    /// - Parameter key: The key given to `NativeblocksFrameState.stateful`.
    @discardableResult
    public func clearFrameState(key: String) -> NativeblocksManager {
        injector.runtimeClient.frameClient.clearFrameState(stateKey: key)
        return self
    }

    /// Throws away every kept frame.
    @discardableResult
    public func clearAllFrameStates() -> NativeblocksManager {
        injector.runtimeClient.frameClient.clearAllFrameStates()
        return self
    }

    /// Retrieves the scaffold for the frames asynchronously.
    /// - Returns: The `NativeScaffoldModel` on success, or the underlying error on failure.
    public func getScaffold() async -> Result<NativeScaffoldModel, Error> {
        do {
            return .success(try await injector.runtimeClient.scaffoldClient.getScaffold().toHost())
        } catch {
            return .failure(error)
        }
    }

    /// Sets the language for the frames.
    /// - Parameter languageCode: The ISO 639-1 language code in uppercase (e.g., "EN" for English, "FR" for French).
    /// - Returns: The current instance of `NativeblocksManager` for method chaining.
    @discardableResult
    public func setLanguage(languageCode: String) -> NativeblocksManager {
        injector.localizationUseCase.setLocalization(languageCode: languageCode)
        return self
    }

    /// Translates a key in the currently selected language.
    /// - Parameter key: The translation key.
    /// - Returns: The translated value, or `nil` when the key is unknown.
    public func translate(key: String) -> String? {
        return injector.localizationUseCase.translate(key: key)
    }

    /// Sets global parameters for the Nativeblocks instance.
    /// - Parameter parameters: A variadic list of key–value pairs representing global parameters.
    /// - Returns: The current instance of `NativeblocksManager` for method chaining.
    @discardableResult
    public func setGlobalParameters(_ parameters: (String, String)...) -> NativeblocksManager {
        var internalParams = [String: String]()
        for (key, value) in parameters {
            internalParams[key] = value
        }
        return setGlobalParameters(internalParams)
    }

    /// Sets global parameters for the Nativeblocks instance.
    /// - Parameter parameters: A dictionary of key–value pairs representing global parameters.
    /// - Returns: The current instance of `NativeblocksManager` for method chaining.
    @discardableResult
    public func setGlobalParameters(_ parameters: [String: String]) -> NativeblocksManager {
        injector.runtimeClient.globalParameterClient.set(parameters: parameters)
        return self
    }

    /// Fetches an experiment value for the given key with type-safe conversion.
    /// The method validates the server-returned variable type against the expected type and returns
    /// the default value on any error or type mismatch.
    ///
    /// - Parameters:
    ///   - key: The experiment key to fetch.
    ///   - defaultValue: The default value to return if the experiment is not found or on error.
    ///                   This also determines the expected return type.
    ///   - cacheTTL: Cache time-to-live in milliseconds. Defaults to 24 hours.
    ///               Pass `nil` for no expiration, `0` to always fetch fresh.
    /// - Returns: The experiment value converted to the appropriate type, or the default value on error.
    ///
    /// Supported types:
    /// - `String` — maps to `STRING` (or `JSON`, returned raw)
    /// - `Int`, `Float`, `Double` — map to `NUMBER`
    /// - `Bool` — maps to `BOOLEAN`
    ///
    /// Example usage:
    /// ```swift
    /// let isFeatureEnabled = await manager.getExperiment(key: "new_feature", defaultValue: false)
    /// let maxRetries = await manager.getExperiment(key: "max_retries", defaultValue: 3)
    /// // Custom cache TTL (1 hour)
    /// let feature = await manager.getExperiment(key: "feature_flag", defaultValue: true, cacheTTL: 3_600_000)
    /// ```
    public func getExperiment<T>(key: String, defaultValue: T, cacheTTL: Int64? = 24 * 60 * 60 * 1000) async -> T {
        guard let result = await injector.experimentUseCase.get(key: key, cacheTTL: cacheTTL) else {
            return defaultValue
        }
        return convertExperimentValue(
            value: result.value,
            variableType: result.variableType,
            defaultValue: defaultValue
        )
    }

    /// Converts the experiment value string to the appropriate type based on the default value type.
    private func convertExperimentValue<T>(value: String, variableType: String, defaultValue: T) -> T {
        switch defaultValue {
        case is String:
            // A JSON experiment is handed back as its raw payload.
            guard variableType == "STRING" || variableType == "JSON" else { return defaultValue }
            return value as? T ?? defaultValue

        case is Bool:
            guard variableType == "BOOLEAN" else { return defaultValue }
            return (value.lowercased() == "true") as? T ?? defaultValue

        case is Int:
            guard variableType == "NUMBER", let intValue = Int(value) else { return defaultValue }
            return intValue as? T ?? defaultValue

        case is Float:
            guard variableType == "NUMBER", let floatValue = Float(value) else { return defaultValue }
            return floatValue as? T ?? defaultValue

        case is Double:
            guard variableType == "NUMBER", let doubleValue = Double(value) else { return defaultValue }
            return doubleValue as? T ?? defaultValue

        default:
            return defaultValue
        }
    }

    /// Destroys the `NativeblocksManager` and releases all associated resources.
    public func destroy() {
        for kit in kits {
            kit.detach(instanceName: name)
        }
        kits.removeAll()

        for loggerType in loggerTypes {
            removeLogger(instanceName: name, loggerType: loggerType)
        }
        loggerTypes.removeAll()

        NativeCoreSDKInjector.destroy(name: name)

        Self.lock.lock()
        Self.instanceRegistry.removeValue(forKey: name)
        Self.lock.unlock()

        NativeBlockProviderRegistry.remove(name)
        NativeActionProviderRegistry.remove(name)
        NativeTypeProviderRegistry.remove(name)
        actionContractors.removeAll()
    }
}

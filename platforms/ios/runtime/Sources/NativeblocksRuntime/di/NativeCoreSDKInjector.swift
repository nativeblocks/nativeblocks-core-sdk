import Foundation
import NativeblocksRuntimeFFI

internal final class NativeCoreSDKInjector {

    private static var injectorRegistry: [String: NativeCoreSDKInjector] = [:]
    private static let lock = NSLock()

    let instanceName: String
    let runtimeClient: NativeRuntimeClientManager
    let experimentUseCase: ExperimentUseCase
    let localizationUseCase: LocalizationUseCase

    private init(name: String, edition: NativeblocksEdition) throws {
        self.instanceName = name

        // ---- ffi module ----
        let cacheDir = try FileManager.default.url(
            for: .cachesDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: true
        )
        let http = URLSessionHttpClient()
        let runtimeClient = try NativeRuntimeClientManager(
            instanceName: name,
            edition: edition,
            http: http,
            cacheDir: cacheDir.path
        )
        self.runtimeClient = runtimeClient

        // ---- feature module ----
        self.experimentUseCase = ExperimentUseCase(runtimeClient: runtimeClient)
        self.localizationUseCase = LocalizationUseCase(
            runtimeClient: runtimeClient,
            stateManager: runtimeClient.localizationStateManager()
        )
    }

    @MainActor
    func makeFrameViewModel() -> FrameViewModel {
        return FrameViewModel(
            frameStateBridge: FrameStateBridgeImpl(frameStateManager: runtimeClient.frameStateManager()),
            instanceName: instanceName
        )
    }

    static func get(name: String) -> NativeCoreSDKInjector {
        lock.lock()
        defer { lock.unlock() }
        guard let instance = injectorRegistry[name] else {
            fatalError("Please make sure the '\(name)' init function has been called before get")
        }
        return instance
    }

    static func initialize(name: String, edition: NativeblocksEdition) {
        lock.lock()
        defer { lock.unlock() }
        guard injectorRegistry[name] == nil else { return }
        do {
            injectorRegistry[name] = try NativeCoreSDKInjector(name: name, edition: edition)
        } catch {
            fatalError("Nativeblocks '\(name)' could not start its runtime: \(error)")
        }
    }

    static func destroy(name: String) {
        lock.lock()
        defer { lock.unlock() }
        injectorRegistry.removeValue(forKey: name)?.runtimeClient.close()
    }
}

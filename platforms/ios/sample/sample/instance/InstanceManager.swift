import Foundation
import NativeblocksRuntime
import NativeblocksFoundation
import NativeblocksDevkit

final class InstanceManager {

    var instances: [InstanceInfo] { InstanceCatalog.instances }

    private var previewKits: [String: PreviewKit] = [:]

    @discardableResult
    func start(_ instance: InstanceInfo) -> NativeblocksManager {
        let wasRunning = isRunning(instance)
        let manager = NativeblocksManager.initialize(
            name: instance.instanceKey,
            edition: .cloud(
                endpoint: instance.apiUrl,
                apiKey: instance.apiKey,
                developmentMode: instance.developmentMode
            )
        )
        if !wasRunning {
            SampleBlockProvider.provideBlocks(name: instance.instanceKey)
            SampleModifierProvider.provideModifiers(name: instance.instanceKey)
            FoundationProvider.provide(name: instance.instanceKey)
            FoundationTypeProvider.provideTypes(name: instance.instanceKey)
            _ = manager.provideTypeConverter(ShapeStyleType.self, converter: ShapeStyleNativeType())
            provideKit(manager, for: instance)
        }
        return manager
    }

    /// DevKit only runs in development mode, PreviewKit only in production mode.
    private func provideKit(_ manager: NativeblocksManager, for instance: InstanceInfo) {
        if instance.developmentMode {
            let devKit = DevKit.Builder()
                .keepScreenOn()
                .autoConnect()
                .logTracking()
                .build()
            _ = manager.provideKit(devKit)
        } else {
            let previewKit = PreviewKit.Builder()
                .launchOnShake()
                .build()
            previewKits[instance.instanceKey] = previewKit
            _ = manager.provideKit(previewKit)
        }
    }

    /// Opens the PreviewKit parameter form for a production instance.
    func launchPreviewKit(_ instance: InstanceInfo) {
        previewKits[instance.instanceKey]?.launch()
    }

    func isRunning(_ instance: InstanceInfo) -> Bool {
        NativeblocksManager.isInitialized(name: instance.instanceKey)
    }

    func stop(_ instance: InstanceInfo) {
        guard isRunning(instance) else { return }
        NativeblocksManager.getInstance(name: instance.instanceKey).destroy()
        previewKits[instance.instanceKey] = nil
    }

    func stopAll() {
        instances.forEach(stop)
    }
}

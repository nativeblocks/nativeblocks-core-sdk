import Foundation
import NativeblocksRuntime

final class InstanceManager {

    var instances: [InstanceInfo] { InstanceCatalog.instances }

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
            // NativeblocksFoundation has not been ported to NativeblocksRuntime yet, so the
            // foundation blocks a frame refers to fall back to the runtime's placeholder.
        }
        return manager
    }

    func isRunning(_ instance: InstanceInfo) -> Bool {
        NativeblocksManager.isInitialized(name: instance.instanceKey)
    }

    func stop(_ instance: InstanceInfo) {
        guard isRunning(instance) else { return }
        NativeblocksManager.getInstance(name: instance.instanceKey).destroy()
    }

    func stopAll() {
        instances.forEach(stop)
    }
}

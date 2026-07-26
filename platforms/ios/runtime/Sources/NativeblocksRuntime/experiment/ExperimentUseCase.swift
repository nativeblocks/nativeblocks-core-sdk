import Foundation

internal final class ExperimentUseCase {

    private let runtimeClient: NativeRuntimeClientManager

    init(runtimeClient: NativeRuntimeClientManager) {
        self.runtimeClient = runtimeClient
    }

    func get(key: String, cacheTTL: Int64?) async -> (value: String, variableType: String)? {
        guard let result = try? await runtimeClient.experimentClient.getExperiment(key: key, cacheTtl: cacheTTL) else {
            return nil
        }
        return (result.value, result.variableType)
    }
}

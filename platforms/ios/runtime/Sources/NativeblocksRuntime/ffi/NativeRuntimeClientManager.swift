import Foundation
import NativeblocksRuntimeFFI

internal final class NativeRuntimeClientManager {

    private let instanceName: String
    private let runtime: NativeblocksRuntime

    let environment: NativeblocksRuntimeFFI.NativeblocksEnvironment

    let frameClient: FrameClient
    let scaffoldClient: ScaffoldClient
    let experimentClient: ExperimentClient
    let localizationClient: LocalizationClient
    let globalParameterClient: GlobalParameterClient

    init(
        instanceName: String,
        edition: NativeblocksEdition,
        http: HttpClient,
        cacheDir: String
    ) throws {
        self.instanceName = instanceName
        self.environment = edition.toEngineEnvironment(instanceName: instanceName)

        let config = SdkConfig(
            version: SDKConfig.SDK_VERSION,
            platform: SDKConfig.SDK_PLATFORM
        )

        self.runtime = try NativeblocksRuntime(
            environment: environment,
            config: config,
            http: http,
            cacheDir: cacheDir
        )

        self.frameClient = runtime.frameClient()
        self.scaffoldClient = runtime.scaffoldClient()
        self.experimentClient = runtime.experimentClient()
        self.localizationClient = runtime.localizationClient()
        self.globalParameterClient = runtime.globalParameterClient()
    }

    func frameStateManager() -> FrameStateManager {
        return frameClient.stateManager()
    }

    func localizationStateManager() -> LocalizationStateManager {
        return localizationClient.stateManager()
    }

    func close() {
        disposeInstance(instanceName: instanceName)
    }
}

extension NativeblocksEdition {
    fileprivate func toEngineEnvironment(instanceName: String) -> NativeblocksRuntimeFFI.NativeblocksEnvironment {
        switch self {
        case .cloud(let endpoint, let apiKey, let developmentMode):
            return NativeblocksRuntimeFFI.NativeblocksEnvironment(
                instanceName: instanceName,
                endpoint: endpoint,
                apiKey: apiKey,
                developmentMode: developmentMode
            )
        case .community:
            return NativeblocksRuntimeFFI.NativeblocksEnvironment(
                instanceName: instanceName,
                endpoint: "",
                apiKey: "",
                developmentMode: false
            )
        }
    }
}

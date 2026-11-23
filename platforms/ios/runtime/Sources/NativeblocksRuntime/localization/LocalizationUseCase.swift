import Foundation
import NativeblocksRuntimeFFI

internal final class LocalizationUseCase {

    private let runtimeClient: NativeRuntimeClientManager
    private let stateManager: LocalizationStateManager

    init(runtimeClient: NativeRuntimeClientManager, stateManager: LocalizationStateManager) {
        self.runtimeClient = runtimeClient
        self.stateManager = stateManager
    }

    private var client: LocalizationClient {
        return runtimeClient.localizationClient
    }

    func setLocalization(languageCode: String) async {
        client.setLanguageCode(languageCode: languageCode)
        _ = try? await client.getLocalization(languageCode: languageCode)
        Task.detached { [client] in
            try? await client.syncLocalization(languageCode: languageCode)
        }
    }

    func translate(key: String, values: [String: String]) -> String? {
        return stateManager.translate(key: key, values: values)
    }
}

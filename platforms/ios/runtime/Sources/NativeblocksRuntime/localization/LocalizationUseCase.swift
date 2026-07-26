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

    func setLocalization(languageCode: String) {
        client.setLanguageCode(languageCode: languageCode)
        Task.detached { [client] in
            try? await client.syncLocalization(languageCode: languageCode)
        }
    }

    func translate(key: String) -> String? {
        return stateManager.translate(key: key)
    }
}

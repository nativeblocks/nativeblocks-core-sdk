import Foundation

internal final class PreviewKitInjector {

    static let shared = PreviewKitInjector()

    private(set) var environment: PreviewKitEnvironment?

    private init() {}

    func initialize(environment: PreviewKitEnvironment) {
        self.environment = environment
    }

    func destroy() {
        environment = nil
    }
}

import Foundation

internal class DevKitInjector {
    private static var environment: DevKitEnvironment?
    private static var cache: INativeCache?
    private static var remoteSession: IRemoteSession?

    static func initialize(environment: DevKitEnvironment) {
        DevKitInjector.environment = environment
    }

    static func provideCache() -> INativeCache {
        if cache == nil {
            cache = NativeCacheStorage(
                userDefaults: UserDefaults(suiteName: "nativeblocks_devkit") ?? UserDefaults.standard,
                prefix: "DEVKIT_"
            )
        }
        return cache!
    }

    static func provideRemoteSession() -> IRemoteSession {
        if remoteSession == nil {
            remoteSession = RemoteSessionImpl(cache: provideCache())
        }
        return remoteSession!
    }

    static func provideEnvironment() -> DevKitEnvironment {
        return environment!
    }

    static func userAuthorization(endpoint: String, token: String, realtimeEndpoint: String) {
        _ = AuthModule.provideAuthRepository().updateTokenAndEndpoint(endpoint: endpoint, token: token, realtimeEndpoint: realtimeEndpoint)
    }

    static func userLogout() {
        AuthModule.provideAuthRepository().logout()
    }

    static func destroy() {
        DevKitLogger.debug("DevKitInjector", "destroy")
        AuthModule.destroy()
        LiveModule.destroy()
        LoggingModule.destroy()
        remoteSession = nil
        cache = nil
        environment = nil
    }
}

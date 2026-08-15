import Foundation

internal class RemoteSessionImpl: IRemoteSession {
    private let cache: INativeCache

    init(cache: INativeCache) {
        self.cache = cache
    }

    func getEndpoint() -> String {
        return cache.pull(key: "ENDPOINT", defaultValue: "")
    }

    func getToken() -> String {
        return cache.pull(key: "AUTH_TOKEN", defaultValue: "")
    }

    func getRealtimeEndpoint() -> String {
        return cache.pull(key: "REALTIME_ENDPOINT", defaultValue: "")
    }

    func getUsername() -> String {
        return cache.pull(key: "USERNAME", defaultValue: "")
    }

    func getUUID() -> String {
        return cache.pull(key: "UUID", defaultValue: "")
    }
}

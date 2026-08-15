import Combine
import Foundation

internal class AuthRepositoryImpl: AuthRepository {
    private let cache: INativeCache
    private let TAG = "AuthRepository"
    init(cache: INativeCache) {
        self.cache = cache
    }

    func updateTokenAndEndpoint(qrData: String) -> Bool {
        do {
            let data = try JSONDecoder().decode(QrData.self, from: Data(qrData.utf8))
            cache.push(key: "ENDPOINT", value: data.endpoint)
            cache.push(key: "REALTIME_ENDPOINT", value: data.realtimeEndpoint)
            if let username = JWTUtils.decodedBody(data.token)?["email"] as? String {
                cache.push(key: "USERNAME", value: username)
            }
            cache.push(key: "UUID", value: UUID().uuidString)
            cache.push(key: "AUTH_TOKEN", value: data.token)
            return true
        } catch {
            DevKitLogger.error(TAG, "updateTokenAndEndpointQR", error)
            return false
        }
    }

    func updateTokenAndEndpoint(endpoint: String, token: String, realtimeEndpoint: String) -> Bool {
        cache.push(key: "ENDPOINT", value: endpoint)
        cache.push(key: "REALTIME_ENDPOINT", value: realtimeEndpoint)
        if let username = JWTUtils.decodedBody(token)?["email"] as? String {
            cache.push(key: "USERNAME", value: username)
        }
        cache.push(key: "UUID", value: UUID().uuidString)
        cache.push(key: "AUTH_TOKEN", value: token)
        return true
    }

    func authCheck() -> Bool {
        return !cache.pull(key: "AUTH_TOKEN", defaultValue: "").isEmpty
    }

    func logout() {
        cache.delete(key: "ENDPOINT")
        cache.delete(key: "AUTH_TOKEN")
        cache.delete(key: "REALTIME_ENDPOINT")
        cache.delete(key: "USERNAME")
        cache.delete(key: "UUID")
    }

    func token() -> AnyPublisher<String, Never> {
        return cache.onValueChange(key: "AUTH_TOKEN", defaultValue: "")
    }
}

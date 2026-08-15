import Combine

internal protocol AuthRepository {
    func updateTokenAndEndpoint(qrData: String) -> Bool
    func updateTokenAndEndpoint(endpoint: String, token: String, realtimeEndpoint: String) -> Bool
    func authCheck() -> Bool
    func logout()
    func token() -> AnyPublisher<String, Never>
}

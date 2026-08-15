import Foundation

internal protocol IRemoteSession {
    func getEndpoint() -> String
    func getToken() -> String
    func getRealtimeEndpoint() -> String
    func getUsername() -> String
    func getUUID() -> String
}

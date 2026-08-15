import Foundation

internal struct QrData: Codable {
    let endpoint: String
    let token: String
    let realtimeEndpoint: String
}

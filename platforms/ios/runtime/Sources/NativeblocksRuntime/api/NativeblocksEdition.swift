import Foundation

/// Represents the different editions of the Nativeblocks framework.
public enum NativeblocksEdition {
    /// Cloud-based configuration.
    /// - Parameters:
    ///   - endpoint: The API endpoint for fetching frame definitions and other resources.
    ///   - apiKey: The API key for authenticating requests to the server.
    ///   - developmentMode: Indicates if development mode is enabled for testing and debugging purposes.
    case cloud(endpoint: String, apiKey: String, developmentMode: Bool)

    /// Community-based configuration using local frame data.
    /// - Parameter frameData: A dictionary where each key is a route and each value is the corresponding frame URL.
    case community(frameData: [String: String])
}

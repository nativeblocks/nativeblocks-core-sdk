import Foundation
import NativeblocksRuntimeFFI

internal final class URLSessionHttpClient: HttpClient, @unchecked Sendable {

    private static let jsonContentType = "application/json; charset=utf-8"

    private let session: URLSession

    init(session: URLSession = URLSessionHttpClient.makeDefaultSession()) {
        self.session = session
    }

    static func makeDefaultSession() -> URLSession {
        let configuration = URLSessionConfiguration.default
        configuration.timeoutIntervalForRequest = 10
        configuration.timeoutIntervalForResource = 10
        return URLSession(configuration: configuration)
    }

    func get(url: String, headers: [String: String]) async throws -> String {
        var request = try makeRequest(url: url, headers: headers)
        request.httpMethod = "GET"
        return try await execute(request)
    }

    func post(url: String, headers: [String: String], body: String) async throws -> String {
        var request = try makeRequest(url: url, headers: headers)
        request.httpMethod = "POST"
        request.httpBody = body.data(using: .utf8)
        if request.value(forHTTPHeaderField: "Content-Type") == nil {
            request.setValue(Self.jsonContentType, forHTTPHeaderField: "Content-Type")
        }
        return try await execute(request)
    }

    private func makeRequest(url: String, headers: [String: String]) throws -> URLRequest {
        guard let parsed = URL(string: url) else {
            throw NbError.Failure(reason: "Invalid URL: \(url)", errorType: .network, errorCode: nil)
        }
        var request = URLRequest(url: parsed)
        for (key, value) in headers {
            request.setValue(value, forHTTPHeaderField: key)
        }
        return request
    }

    private func execute(_ request: URLRequest) async throws -> String {
        try await uniffiCallbackGuard(.network) {
            let (data, response) = try await session.data(for: request)
            let payload = String(data: data, encoding: .utf8) ?? ""

            guard let http = response as? HTTPURLResponse else { return payload }
            guard (200..<300).contains(http.statusCode) else {
                let url = request.url?.absoluteString ?? ""
                throw NbError.Failure(
                    reason: "HTTP \(http.statusCode) for \(url): \(payload)",
                    errorType: .network,
                    errorCode: String(http.statusCode)
                )
            }
            return payload
        }
    }
}

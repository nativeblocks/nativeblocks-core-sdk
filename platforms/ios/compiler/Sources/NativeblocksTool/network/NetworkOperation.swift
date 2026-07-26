import Foundation

struct AnyEncodable: Encodable {
    private let _encode: (Encoder) throws -> Void

    init<T: Encodable>(_ wrapped: T) {
        _encode = wrapped.encode
    }

    func encode(to encoder: Encoder) throws {
        try _encode(encoder)
    }
}

struct NetworkOperation: Encodable {
    private var operationString: String
    private var headers: [String: String] = [:]

    private var operationName: String?
    private var variables: [String: AnyEncodable]?

    enum CodingKeys: String, CodingKey {
        case variables
        case operationName
        case query
    }

    init(
        _ operationString: String, operationName: String? = nil,
        variables: [String: AnyEncodable]? = nil,
        headers: [String: String] = [:]
    ) {
        self.operationString = operationString
        self.operationName = operationName
        self.variables = variables
        self.headers = headers
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(operationString, forKey: .query)
        try container.encodeIfPresent(operationName, forKey: .operationName)
        try container.encodeIfPresent(variables, forKey: .variables)
    }

    func getURLRequest(endpoint: String, apiKey: String) throws -> URLRequest {
        let url = URL(string: endpoint)!
        var request = URLRequest(url: url)

        request.httpMethod = "POST"
        for header in headers {
            request.setValue(header.value, forHTTPHeaderField: header.key)
        }
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("Bearer \(apiKey)", forHTTPHeaderField: "authorization")
        request.httpBody = try JSONEncoder().encode(self)

        return request
    }
}

extension URLSession {
    func getData(from urlRequest: URLRequest) async throws -> (Data, URLResponse) {
        try await withCheckedThrowingContinuation { continuation in
            let task = self.dataTask(with: urlRequest) { data, response, error in
                guard let data = data, let response = response else {
                    let error = error ?? URLError(.badServerResponse)
                    return continuation.resume(throwing: error)
                }
                continuation.resume(returning: (data, response))
            }
            task.resume()
        }
    }
}

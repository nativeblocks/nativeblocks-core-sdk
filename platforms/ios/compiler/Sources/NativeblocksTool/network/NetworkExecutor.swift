import Foundation

class NetworkExecutor {
    private static var instance: NetworkExecutor!

    private static var endpoint: String?
    private static var apiKey: String?

    static func initialize(endpoint: String, apiKey: String) {
        self.endpoint = endpoint
        self.apiKey = apiKey
    }

    public static func getInstance() -> NetworkExecutor {
        if instance == nil {
            instance = NetworkExecutor()
        }
        return instance!
    }

    func performOperation<Output: Codable>(
        _ operation: NetworkOperation, completion: @escaping (ResultModel<Output>) -> Void
    ) {
        guard let endpoint = NetworkExecutor.endpoint, !endpoint.isEmpty else {
            completion(ResultModel.error(ErrorModel(message: "Please provide a valid endpoint", errorType: "NETWORK")))
            return
        }

        guard let apiKey = NetworkExecutor.apiKey, !apiKey.isEmpty else {
            completion(ResultModel.error(ErrorModel(message: "Please provide a valid apiKey", errorType: "NETWORK")))
            return
        }

        let decoder = JSONDecoder()
        do {
            let request: URLRequest = try operation.getURLRequest(endpoint: endpoint, apiKey: apiKey)
            let task = URLSession.shared.dataTask(with: request) { data, _, error in
                if let error = error {
                    completion(ResultModel.error(ErrorModel(message: error.localizedDescription, errorType: "UN_KNOWN")))
                    return
                }
                guard let data = data else {
                    completion(ResultModel.error(ErrorModel(message: "No data received", errorType: "NETWORK")))
                    return
                }
                do {
                    let responseModel = try decoder.decode(NetworkResult<Output>.self, from: data)
                    if let errors = responseModel.errors {
                        completion(
                            ResultModel.error(
                                ErrorModel(message: errors.first?.message, errorType: errors.first?.extensions.classification)))
                    } else if let resultData = responseModel.data {
                        completion(ResultModel.success(resultData))
                    } else {
                        completion(ResultModel.error(ErrorModel(message: "There’s an issue. Please try again later", errorType: "NETWORK")))
                    }
                } catch {
                    completion(
                        ResultModel.error(
                            ErrorModel(message: "Failed to parse response: \(error.localizedDescription)", errorType: "PARSING")))
                }
            }
            task.resume()
        } catch {
            completion(ResultModel.error(ErrorModel(message: error.localizedDescription, errorType: "REQUEST_CREATION")))
        }
    }
}

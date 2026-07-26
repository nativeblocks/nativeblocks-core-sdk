import Foundation

struct ErrorResult: Codable {
    let message: String
    let extensions: ErrorExtensions

    struct ErrorExtensions: Codable {
        let classification: String
    }
}

struct NetworkResult<T: Codable>: Codable {
    let errors: [ErrorResult]?
    let data: T?
}

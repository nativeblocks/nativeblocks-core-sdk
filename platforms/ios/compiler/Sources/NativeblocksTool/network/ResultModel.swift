import Foundation

enum ResultModel<V> {
    case success(V)
    case error(ErrorModel)
}

struct ErrorModel: Codable, Error {
    let message: String?
    let errorType: String?
}

extension ResultModel {
    func doOnSuccess(ifSuccess: (V) -> Void) {
        if case .success(let value) = self {
            ifSuccess(value)
        }
    }

    func doOnError(ifError: (ErrorModel) -> Void) {
        if case .error(let error) = self {
            ifError(error)
        }
    }

    func map<R>(isSuccess: (V) -> R) -> ResultModel<R> {
        switch self {
        case .success(let value):
            return .success(isSuccess(value))
        case .error(let error):
            return .error(error)
        }
    }

    func execute<R>(ifSuccess: (V) -> R, ifError: (ErrorModel) -> R) -> R {
        switch self {
        case .success(let value):
            return ifSuccess(value)
        case .error(let error):
            return ifError(error)
        }
    }

    func executeAsync<R>(ifSuccess: (V) -> R, ifError: (ErrorModel) -> R) -> R {
        switch self {
        case .success(let value):
            return ifSuccess(value)
        case .error(let error):
            return ifError(error)
        }
    }
}

import Foundation
import NativeblocksRuntimeFFI

internal func uniffiCallbackGuard<R>(_ errorType: ErrorType, _ block: () throws -> R) throws -> R {
    do {
        return try block()
    } catch let error as NbError {
        throw error
    } catch {
        throw NbError.Failure(
            reason: error.localizedDescription,
            errorType: errorType,
            errorCode: nil
        )
    }
}

internal func uniffiCallbackGuard<R>(_ errorType: ErrorType, _ block: () async throws -> R) async throws -> R {
    do {
        return try await block()
    } catch let error as NbError {
        throw error
    } catch {
        throw NbError.Failure(
            reason: error.localizedDescription,
            errorType: errorType,
            errorCode: nil
        )
    }
}

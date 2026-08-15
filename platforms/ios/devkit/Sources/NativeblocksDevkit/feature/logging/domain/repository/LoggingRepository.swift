import Foundation
import NativeblocksRuntime

internal protocol LoggingRepository {

    func sendLog(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String]
    ) async throws
}

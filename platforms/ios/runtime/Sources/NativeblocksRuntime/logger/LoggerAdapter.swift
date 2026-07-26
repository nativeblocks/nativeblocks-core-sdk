import Foundation
import NativeblocksRuntimeFFI

internal final class LoggerAdapter: NativeblocksRuntimeFFI.Logger, @unchecked Sendable {

    private let delegate: any INativeLogger

    init(delegate: any INativeLogger) {
        self.delegate = delegate
    }

    func log(
        level: RuntimeFFILoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String]
    ) {
        delegate.logWithContext(
            level: level.toDomain(),
            event: event,
            message: message,
            parameters: parameters
        )
    }
}

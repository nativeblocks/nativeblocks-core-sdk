import Foundation

/// Protocol that defines the behavior for a native logger.
/// The `INativeLogger` protocol is used for logging events within the Nativeblocks SDK.
public protocol INativeLogger {
    /// Logs an event with the specified name and parameters.
    /// - Parameters:
    ///   - level: The severity of the log (DEBUG, INFO, WARNING, ERROR).
    ///   - event: The event type (from LoggerEventType).
    ///   - message: A human-readable message for the log.
    ///   - parameters: Optional key-value pairs with event-specific data.
    func log(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String],
    )
}

extension INativeLogger {
    func logWithContext(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String],
    ) {
        var combinedParameters = parameters
        combinedParameters["SDK-Version"] = SDKConfig.SDK_VERSION
        combinedParameters["SDK-Platform"] = SDKConfig.SDK_PLATFORM

        log(level: level, event: event, message: message, parameters: combinedParameters)
    }
}

public enum LoggerEventLevel: String {
    case DEBUG
    case INFO
    case WARNING
    case ERROR
}

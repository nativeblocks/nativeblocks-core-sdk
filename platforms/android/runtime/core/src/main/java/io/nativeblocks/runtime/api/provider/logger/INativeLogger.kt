package io.nativeblocks.runtime.api.provider.logger

/**
 * Defines the contract for logging events within the native framework.
 */
interface INativeLogger {

    /**
     * Logs a structured event.
     *
     * @param level The severity of the log (INFO, WARNING, ERROR).
     * @param event The event type (from LoggerEventType).
     * @param message A human-readable message for the log.
     * @param parameters Optional key-value pairs with event-specific data.
     */
    fun log(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: Map<String, String> = emptyMap(),
    )
}

enum class LoggerEventLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}
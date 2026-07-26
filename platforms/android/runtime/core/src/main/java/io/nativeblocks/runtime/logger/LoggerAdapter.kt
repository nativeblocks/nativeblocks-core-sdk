package io.nativeblocks.runtime.logger

import io.nativeblocks.runtime.api.provider.logger.INativeLogger
import io.nativeblocks.runtime.ffi.Logger
import io.nativeblocks.runtime.ffi.LoggerEventLevel
import io.nativeblocks.runtime.ffi.toDomain

internal class LoggerAdapter(
    private val delegate: INativeLogger,
) : Logger {

    override fun log(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: Map<String, String>,
    ) {
        delegate.log(
            level = level.toDomain(),
            event = event,
            message = message,
            parameters = parameters,
        )
    }
}
package io.nativeblocks.devkit.feature.logging.domain.repository

import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel

internal interface LoggingRepository {

    suspend fun sendLog(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: Map<String, String>
    )
}

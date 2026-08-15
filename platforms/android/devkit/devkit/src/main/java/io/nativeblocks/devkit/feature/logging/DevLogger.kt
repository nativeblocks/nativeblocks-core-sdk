package io.nativeblocks.devkit.feature.logging

import io.nativeblocks.runtime.api.provider.logger.INativeLogger
import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel
import io.nativeblocks.devkit.feature.logging.domain.repository.LoggingRepository
import io.nativeblocks.devkit.util.DevKitLogger
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch

internal class DevLogger(
    private val loggingRepository: LoggingRepository
) : INativeLogger {

    companion object {
        private val TAG = DevLogger::class.java.simpleName
    }

    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)

    override fun log(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: Map<String, String>
    ) {
        DevKitLogger.d(TAG, "Logging event: $event, level: $level, message: $message")

        scope.launch {
            try {
                loggingRepository.sendLog(
                    level = level,
                    event = event,
                    message = message,
                    parameters = parameters
                )
            } catch (e: Exception) {
                DevKitLogger.e(TAG, "Failed to send log: ${e.message}", e)
            }
        }
    }
}

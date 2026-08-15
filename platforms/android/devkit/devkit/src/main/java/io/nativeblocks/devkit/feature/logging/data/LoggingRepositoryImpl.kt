package io.nativeblocks.devkit.feature.logging.data

import io.nativeblocks.devkit.feature.live.data.SocketClient
import io.nativeblocks.devkit.feature.live.data.LogEventMessage
import io.nativeblocks.devkit.feature.logging.domain.repository.LoggingRepository
import io.nativeblocks.devkit.session.DevKitEnvironment
import io.nativeblocks.devkit.session.IRemoteSession
import io.nativeblocks.devkit.util.DevKitLogger
import io.nativeblocks.devkit.util.JWT
import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel

internal class LoggingRepositoryImpl(
    private val signalingClient: SocketClient,
    private val kitEnvironment: DevKitEnvironment,
    private val remoteSession: IRemoteSession,
) : LoggingRepository {

    companion object {
        private val TAG = LoggingRepositoryImpl::class.java.simpleName
    }

    private val uuid: String
        get() = remoteSession.getUUID()

    private val projectId: String by lazy {
        JWT.decodedBody(kitEnvironment.apiKey)?.getString("id") ?: ""
    }

    override suspend fun sendLog(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: Map<String, String>
    ) {
        try {
            signalingClient.send(
                LogEventMessage(
                    clientId = uuid,
                    roomId = projectId,
                    level = level.name,
                    event = event,
                    message = message,
                    parameters = parameters
                )
            )
            DevKitLogger.d(TAG, "Log sent: event=$event, level=$level")
        } catch (e: Exception) {
            DevKitLogger.e(TAG, "Failed to send log message: ${e.message}", e)
        }
    }
}

package io.nativeblocks.devkit.feature.live.data

import android.content.Context
import android.os.Build
import io.nativeblocks.devkit.feature.live.domain.model.ConnectionState
import io.nativeblocks.devkit.feature.live.domain.repository.LiveRepository
import io.nativeblocks.devkit.session.DevKitEnvironment
import io.nativeblocks.devkit.session.IRemoteSession
import io.nativeblocks.devkit.util.DevKitLogger
import io.nativeblocks.devkit.util.JWT
import io.nativeblocks.devkit.util.registerOrientationChange
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch

internal class LiveRepositoryImpl(
    private val context: Context,
    private val signalingClient: SocketClient,
    private val kitEnvironment: DevKitEnvironment,
    private val remoteSession: IRemoteSession,
) : LiveRepository {

    companion object {
        private val TAG = LiveRepository::class.java.simpleName
    }

    private val _connectionState = MutableStateFlow(ConnectionState.NotConnected)
    private val _hotReload = MutableSharedFlow<String>()

    private val isScreenSharing
        get() = kitEnvironment.screenSharing

    private val username: String
        get() = remoteSession.getUsername()
    private val uuid: String
        get() = remoteSession.getUUID()
    private val projectId: String by lazy {
        JWT.decodedBody(kitEnvironment.apiKey)?.getString("id") ?: ""
    }

    private var streamingToken: String = ""
    private var streamingChannel: String = ""
    private var streamingId: String = ""

    private fun getDeviceName(): String {
        val manufacture = Build.MANUFACTURER
        val model = Build.MODEL
        return if (model.startsWith(manufacture)) {
            model
        } else {
            "$manufacture $model"
        }
    }

    override suspend fun start() {
        DevKitLogger.d(TAG, "registerOrientationChange")
        DevKitLogger.d(TAG, "start")
        startSignal()
    }

    override suspend fun end() {
        DevKitLogger.d(TAG, "end")
        endSession()
    }

    override suspend fun orientedChange() {
        DevKitLogger.d(TAG, "orientedChange")
        registerOrientationChange(context).collect {
            if (isScreenSharing) {
                // NO-OP -> adjustVideoSize()
            }
        }
    }

    override fun connectionState() = _connectionState.asStateFlow()

    override fun hotReload() = _hotReload.asSharedFlow()

    private suspend fun startSignal() {
        DevKitLogger.d(TAG, "startSignal")
        signalingClient.signals(remoteSession.getRealtimeEndpoint()).catch {
            DevKitLogger.e(TAG, "Signals error: ${it.message}")
        }.collect { state ->
            when (state) {
                is WebSocketState.ReceiveSignal -> {
                    DevKitLogger.d(TAG, "WebSocket ReceiveSignal ${state.signal}")
                    when (val signal = state.signal) {
                        is StartSessionMessage -> {
                            DevKitLogger.d(TAG, "StartSession acknowledged")
                        }

                        is EndSessionMessage -> {
                            streamingToken = ""
                            streamingChannel = ""
                            streamingId = ""
                        }

                        is HotReloadMessage -> {
                            val frameRoute = signal.frameRoute ?: ""
                            _hotReload.emit(frameRoute)
                        }

                        is AvailableTargetsMessage -> {
                            // SDK doesn't need to handle AvailableTargets (Studio-only message)
                            DevKitLogger.d(TAG, "AvailableTargets received (Studio-only message)")
                        }

                        is LogEventMessage -> {
                            // SDK doesn't need to handle log events from Studio (optional for bidirectional logging)
                            DevKitLogger.d(TAG, "Log event received from Studio: ${signal.message}")
                        }
                    }
                }

                WebSocketState.Connecting -> {
                    DevKitLogger.d(TAG, "WebSocket Connecting")
                    _connectionState.emit(ConnectionState.Connecting)
                }

                WebSocketState.Connected -> {
                    DevKitLogger.d(TAG, "WebSocket Connected")
                    _connectionState.emit(ConnectionState.Connected)
                    startSession()
                }

                WebSocketState.Disconnected -> {
                    _connectionState.emit(ConnectionState.NotConnected)
                    DevKitLogger.d(TAG, "WebSocket Disconnected")
                }

                is WebSocketState.Error -> {
                    DevKitLogger.e(TAG, "WebSocket Error: ${state.message}", state.error)
                    _connectionState.emit(ConnectionState.NotConnected)
                }
            }
        }
    }

    private suspend fun startSession() {
        DevKitLogger.d(TAG, "startSession")
        if (isScreenSharing) {
            // NO-OP -> adjustVideoSize()
        }
        signalingClient.send(
            StartSessionMessage(
                clientType = SignalingClientType.SDK,
                clientId = uuid,
                roomId = projectId,
                username = username,
                deviceName = getDeviceName(),
                hasSharingScreen = isScreenSharing
            )
        )
    }

    private suspend fun endSession() {
        DevKitLogger.d(TAG, "endSession")
        signalingClient.send(
            EndSessionMessage(
                clientId = uuid,
                roomId = projectId
            )
        )
        streamingToken = ""
        streamingChannel = ""
        streamingId = ""
        _connectionState.emit(ConnectionState.NotConnected)
        if (isScreenSharing) {
            // NO-OP -> endStreaming()
        }
    }
}
package io.nativeblocks.devkit.feature.live.data

import io.ktor.client.HttpClient
import io.ktor.client.plugins.websocket.webSocket
import io.ktor.websocket.CloseReason
import io.ktor.websocket.Frame
import io.ktor.websocket.WebSocketSession
import io.ktor.websocket.close
import io.ktor.websocket.readText
import io.nativeblocks.devkit.util.DevKitLogger
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.channels.consumeEach
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import kotlinx.coroutines.isActive
import kotlinx.coroutines.launch
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.decodeFromJsonElement
import kotlinx.serialization.json.jsonPrimitive

internal class SocketClient(private val client: HttpClient) {

    companion object {
        private val TAG = SocketClient::class.java.simpleName
        private val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
        }
    }

    private var webSocketSession: WebSocketSession? = null
    private val reconnectDelay = 5000L

    suspend fun send(signalingModel: SignalingModel) {
        try {
            val frame = when (signalingModel) {
                is StartSessionMessage -> json.encodeToString(signalingModel)
                is EndSessionMessage -> json.encodeToString(signalingModel)
                is HotReloadMessage -> json.encodeToString(signalingModel)
                is AvailableTargetsMessage -> json.encodeToString(signalingModel)
                is LogEventMessage -> json.encodeToString(signalingModel)
            }
            DevKitLogger.d(TAG, "Sending => frame: $frame")
            webSocketSession?.send(Frame.Text(frame))
        } catch (e: Exception) {
            DevKitLogger.e(TAG, "Sending error: ${e.message}")
        }
    }

    fun signals(url: String): Flow<WebSocketState> = callbackFlow {
        DevKitLogger.d(TAG, "State: Start")
        var webSocketSession: WebSocketSession? = null
        while (isActive) {
            DevKitLogger.d(TAG, "State: Connecting")
            trySend(WebSocketState.Connecting)
            try {
                client.webSocket(url) {
                    webSocketSession = this
                    this@SocketClient.webSocketSession = this
                    DevKitLogger.d(TAG, "State: Connected")
                    trySend(WebSocketState.Connected)
                    val receiveJob = launch {
                        incoming.consumeEach { frame ->
                            when (frame) {
                                is Frame.Text -> {
                                    val textFrame = frame.readText()
                                    DevKitLogger.d(TAG, "Receive <= frame: $textFrame")
                                    deserializeSignal(textFrame)?.let {
                                        trySend(WebSocketState.ReceiveSignal(it))
                                    }
                                }

                                is Frame.Close -> {
                                    DevKitLogger.d(TAG, "Receive <= Close")
                                    DevKitLogger.d(TAG, "State: Disconnected")
                                    trySend(WebSocketState.Disconnected)
                                    close(CloseReason(CloseReason.Codes.NORMAL, "Disconnecting"))
                                }

                                else -> Unit
                            }
                        }
                    }

                    awaitClose {
                        DevKitLogger.d(TAG, "Websocket awaitClose ")
                        receiveJob.cancel()
                        launch {
                            webSocketSession?.close(
                                CloseReason(CloseReason.Codes.NORMAL, "Disconnecting")
                            )
                        }
                        DevKitLogger.d(TAG, "State: Disconnected")
                        trySend(WebSocketState.Disconnected)
                    }
                }
            } catch (e: Exception) {
                DevKitLogger.e(TAG, "Failed to connect", e)
                trySend(WebSocketState.Error(e.message ?: "Failed to connect", e))
                if (isActive) {
                    DevKitLogger.e(TAG, "Reconnect")
                    trySend(WebSocketState.Connecting)
                    delay(reconnectDelay)
                }
            }
        }

        awaitClose {
            DevKitLogger.d(TAG, "signals callback awaitClose")
            launch {
                webSocketSession?.close(CloseReason(CloseReason.Codes.NORMAL, "Disconnecting"))
            }
            trySend(WebSocketState.Disconnected)
        }
    }

    private fun deserializeSignal(jsonString: String): SignalingModel? {
        return try {
            val jsonElement = json.parseToJsonElement(jsonString) as JsonObject
            when (val type = jsonElement["type"]?.jsonPrimitive?.content) {
                SignalingTypeModel.START_SESSION.name -> json.decodeFromJsonElement<StartSessionMessage>(jsonElement)
                SignalingTypeModel.END_SESSION.name -> json.decodeFromJsonElement<EndSessionMessage>(jsonElement)
                SignalingTypeModel.HOT_RELOAD.name -> json.decodeFromJsonElement<HotReloadMessage>(jsonElement)
                SignalingTypeModel.AVAILABLE_TARGETS.name -> json.decodeFromJsonElement<AvailableTargetsMessage>(jsonElement)
                SignalingTypeModel.LOG_EVENT.name -> json.decodeFromJsonElement<LogEventMessage>(jsonElement)
                else -> {
                    DevKitLogger.e(TAG, "Unknown signal type: $type")
                    null
                }
            }
        } catch (e: Exception) {
            DevKitLogger.e(TAG, "Failed to deserialize signal: ${e.message}", e)
            null
        }
    }
}
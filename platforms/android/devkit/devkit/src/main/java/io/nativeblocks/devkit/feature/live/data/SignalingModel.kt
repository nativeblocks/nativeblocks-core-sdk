package io.nativeblocks.devkit.feature.live.data

import kotlinx.serialization.Serializable

internal enum class SignalingTypeModel {
    START_SESSION,
    END_SESSION,
    HOT_RELOAD,
    AVAILABLE_TARGETS,
    LOG_EVENT
}

internal enum class SignalingClientType {
    SDK,
    STUDIO
}

@Serializable
internal sealed interface SignalingModel {
    val type: SignalingTypeModel
    val roomId: String
}

@Serializable
internal data class StartSessionMessage(
    override val type: SignalingTypeModel = SignalingTypeModel.START_SESSION,
    val clientType: SignalingClientType,
    val clientId: String,
    override val roomId: String,
    val username: String,
    val deviceName: String? = null,
    val hasSharingScreen: Boolean? = null
) : SignalingModel

@Serializable
internal data class EndSessionMessage(
    override val type: SignalingTypeModel = SignalingTypeModel.END_SESSION,
    val clientId: String,
    override val roomId: String
) : SignalingModel

@Serializable
internal data class HotReloadMessage(
    override val type: SignalingTypeModel = SignalingTypeModel.HOT_RELOAD,
    val clientType: SignalingClientType,
    val clientId: String,
    override val roomId: String,
    val username: String,
    val target: String,
    val frameRoute: String? = null
) : SignalingModel

@Serializable
internal data class TargetInfo(
    val uuid: String,
    val deviceName: String,
    val hasSharingScreen: Boolean
)

@Serializable
internal data class AvailableTargetsMessage(
    override val type: SignalingTypeModel = SignalingTypeModel.AVAILABLE_TARGETS,
    override val roomId: String,
    val availableTargets: List<TargetInfo>
) : SignalingModel

@Serializable
internal data class LogEventMessage(
    override val type: SignalingTypeModel = SignalingTypeModel.LOG_EVENT,
    val clientId: String,
    override val roomId: String,
    val level: String,
    val event: String,
    val message: String,
    val parameters: Map<String, String>,
    val timestamp: Long = System.currentTimeMillis()
) : SignalingModel

internal sealed class WebSocketState {
    data object Connecting : WebSocketState()
    data object Connected : WebSocketState()
    data object Disconnected : WebSocketState()
    data class ReceiveSignal(val signal: SignalingModel) : WebSocketState()
    data class Error(val message: String, val error: Exception) : WebSocketState()
}
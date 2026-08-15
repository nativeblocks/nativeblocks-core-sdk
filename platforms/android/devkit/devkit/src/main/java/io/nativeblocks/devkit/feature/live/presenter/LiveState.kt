package io.nativeblocks.devkit.feature.live.presenter

internal data class LiveState(val notificationState: NotificationState) {

    companion object {
        fun initial() = LiveState(
            notificationState = NotificationState.AUTH_REQUIRE
        )
    }
}

internal enum class NotificationState {
    CONNECT,
    CONNECTING,
    NOT_CONNECT,
    AUTH_REQUIRE
}

internal enum class Action {
    START,
    CONNECT,
    DISCONNECT,
    LOGOUT
}

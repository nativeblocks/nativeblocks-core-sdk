internal struct LiveState {
    var notificationState: NotificationState

    static func initial() -> LiveState {
        return LiveState(notificationState: .none)
    }
}

internal enum NotificationState: String {
    case none = "NONE"
    case connect = "CONNECT"
    case connecting = "CONNECTING"
    case notConnect = "NOT_CONNECT"
    case authRequire = "AUTH_REQUIRE"
}

internal enum Action {
    case auth
    case start
    case connect
    case disconnect
    case logout
}

enum LiveServiceAction: String {
    case auth = "AUTH"
    case start = "START"
    case connect = "CONNECT"
    case disconnect = "DISCONNECT"
    case logout = "LOGOUT"
    case stop = "STOP"
    case notificationDeleted = "NOTIFICATION_DELETED"
}

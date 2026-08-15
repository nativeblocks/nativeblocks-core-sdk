import Foundation

internal enum SignalingTypeModel: String, Codable {
    case START_SESSION
    case END_SESSION
    case HOT_RELOAD
    case AVAILABLE_TARGETS
    case LOG_EVENT
}

internal enum SignalingClientType: String, Codable {
    case SDK
    case STUDIO
}

internal enum WebSocketState {
    case connecting
    case connected
    case disconnected
    case receiveSignal(SignalingModel)
    case error(message: String, error: Error)
}

internal protocol SignalingModel: Codable {
    var type: SignalingTypeModel { get }
    var roomId: String { get }
}

internal struct StartSessionMessage: SignalingModel {
    let type: SignalingTypeModel
    let clientType: SignalingClientType
    let clientId: String
    let roomId: String
    let username: String
    let deviceName: String?
    let hasSharingScreen: Bool?

    init(
        clientType: SignalingClientType,
        clientId: String,
        roomId: String,
        username: String,
        deviceName: String? = nil,
        hasSharingScreen: Bool? = nil
    ) {
        self.type = .START_SESSION
        self.clientType = clientType
        self.clientId = clientId
        self.roomId = roomId
        self.username = username
        self.deviceName = deviceName
        self.hasSharingScreen = hasSharingScreen
    }
}

internal struct EndSessionMessage: SignalingModel {
    let type: SignalingTypeModel
    let clientId: String
    let roomId: String

    init(clientId: String, roomId: String) {
        self.type = .END_SESSION
        self.clientId = clientId
        self.roomId = roomId
    }
}

internal struct HotReloadMessage: SignalingModel {
    let type: SignalingTypeModel
    let clientType: SignalingClientType
    let clientId: String
    let roomId: String
    let username: String
    let target: String
    let frameRoute: String?

    init(
        clientType: SignalingClientType,
        clientId: String,
        roomId: String,
        username: String,
        target: String,
        frameRoute: String? = nil
    ) {
        self.type = .HOT_RELOAD
        self.clientType = clientType
        self.clientId = clientId
        self.roomId = roomId
        self.username = username
        self.target = target
        self.frameRoute = frameRoute
    }
}

internal struct TargetInfo: Codable {
    let uuid: String
    let deviceName: String
    let hasSharingScreen: Bool
}

internal struct AvailableTargetsMessage: SignalingModel {
    let type: SignalingTypeModel
    let roomId: String
    let availableTargets: [TargetInfo]

    init(roomId: String, availableTargets: [TargetInfo]) {
        self.type = .AVAILABLE_TARGETS
        self.roomId = roomId
        self.availableTargets = availableTargets
    }
}

internal struct LogEventMessage: SignalingModel {
    let type: SignalingTypeModel
    let clientId: String
    let roomId: String
    let level: String
    let event: String
    let message: String
    let parameters: [String: String]
    let timestamp: Int64

    init(
        clientId: String,
        roomId: String,
        level: String,
        event: String,
        message: String,
        parameters: [String: String]
    ) {
        self.type = .LOG_EVENT
        self.clientId = clientId
        self.roomId = roomId
        self.level = level
        self.event = event
        self.message = message
        self.parameters = parameters
        self.timestamp = Int64(Date().timeIntervalSince1970 * 1000)
    }
}

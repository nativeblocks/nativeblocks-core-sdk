import Combine
import Foundation

internal class LiveRepositoryImpl: NSObject, LiveRepository {
    private static let TAG = "LiveRepository"
    private let signalingClient: SocketClient
    private let kitEnvironment: DevKitEnvironment
    private let remoteSession: IRemoteSession

    private let connectionStateSubject = CurrentValueSubject<ConnectionState, Never>(.notConnected)
    private let hotReloadSubject = PassthroughSubject<String, Never>()

    private var streamingToken: String = ""
    private var streamingChannel: String = ""
    private var streamingId: String = ""

    private var cancellables = Set<AnyCancellable>()
    private var orientationChangeListener: OrientationUtils? = nil

    init(
        signalingClient: SocketClient,
        kitEnvironment: DevKitEnvironment,
        remoteSession: IRemoteSession
    ) {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "init")
        self.signalingClient = signalingClient
        self.kitEnvironment = kitEnvironment
        self.remoteSession = remoteSession
        self.orientationChangeListener = OrientationUtils()
    }

    private var isScreenSharing: Bool { kitEnvironment.screenSharing }

    private var username: String { remoteSession.getUsername() }

    private var uuid: String { remoteSession.getUUID() }

    private var projectId: String {
        JWTUtils.decodedBody(kitEnvironment.apiKey)?["id"] as? String ?? ""
    }

    func start() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "start")
        signalingClient.signals().sink(
            receiveCompletion: { completion in
                if case .failure(let error) = completion {
                    DevKitLogger.debug(LiveRepositoryImpl.TAG, "Signals error: \(error.localizedDescription)")
                }
            },
            receiveValue: { [weak self] state in
                self?.handleWebSocketState(state)
            }
        )
        .store(in: &cancellables)

        signalingClient.connect(urlString: remoteSession.getRealtimeEndpoint())
    }

    func end() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "end")
        endSession()
    }

    func orientedChange() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "orientedChange")
        orientationChangeListener?.register { _ in
            if self.isScreenSharing {
                // NO-OP -> adjustVideoSize()
            }
        }
    }

    func connectionState() -> AnyPublisher<ConnectionState, Never> {
        return connectionStateSubject.eraseToAnyPublisher()
    }

    func hotReload() -> AnyPublisher<String, Never> {
        return hotReloadSubject.eraseToAnyPublisher()
    }

    private func handleWebSocketState(_ state: WebSocketState) {
        switch state {
        case .connecting:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "WebSocket Connecting")
            connectionStateSubject.send(.connecting)
        case .connected:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "WebSocket Connected")
            connectionStateSubject.send(.connected)
            startSession()
        case .disconnected:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "WebSocket Disconnected")
            connectionStateSubject.send(.notConnected)
        case .receiveSignal(let signal):
            handleReceivedSignal(signal)
        case .error(let message, let error):
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "WebSocket Error: \(message), \(error.localizedDescription)")
        //            connectionStateSubject.send(.notConnected)
        }
    }

    private func handleReceivedSignal(_ signal: SignalingModel) {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "Received signal type: \(signal.type)")

        switch signal {
        case let msg as StartSessionMessage:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "StartSession acknowledged for client: \(msg.clientId)")
        // SDK doesn't need special handling for StartSession acknowledgment

        case _ as EndSessionMessage:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "EndSession received")
            streamingToken = ""
            streamingChannel = ""
            streamingId = ""

        case let msg as HotReloadMessage:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "HotReload received")
            let frameRoute = msg.frameRoute ?? ""
            hotReloadSubject.send(frameRoute)

        case _ as AvailableTargetsMessage:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "AvailableTargets received (Studio-only message)")
        // SDK doesn't need to handle AvailableTargets (Studio-only message)

        case let msg as LogEventMessage:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "Log event received from Studio: \(msg.message)")
        // SDK doesn't need to handle log events from Studio (optional for bidirectional logging)

        default:
            DevKitLogger.debug(LiveRepositoryImpl.TAG, "Unknown signal type")
        }
    }

    private func startSession() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "startSession")
        if isScreenSharing {
            // NO-OP -> adjustVideoSize()
        }

        let message = StartSessionMessage(
            clientType: .SDK,
            clientId: uuid,
            roomId: projectId,
            username: username,
            deviceName: DeviceInfo.getDeviceName(),
            hasSharingScreen: isScreenSharing
        )
        signalingClient.send(message: message)
    }

    private func endSession() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "endSession")
        let message = EndSessionMessage(
            clientId: uuid,
            roomId: projectId
        )
        signalingClient.send(message: message)

        streamingToken = ""
        streamingChannel = ""
        streamingId = ""

        connectionStateSubject.send(.notConnected)
        if isScreenSharing {
            // NO-OP -> endStreaming()
        }
        signalingClient.disconnect()
    }

    func cleanUp() {
        DevKitLogger.debug(LiveRepositoryImpl.TAG, "cleanUp")
        orientationChangeListener?.unregister()
        orientationChangeListener = nil
        cancellables.forEach { $0.cancel() }
        cancellables.removeAll()
        signalingClient.disconnect()
    }
}

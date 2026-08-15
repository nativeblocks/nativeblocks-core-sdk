import Combine
import Foundation

internal class SocketClient: NSObject, URLSessionWebSocketDelegate {
    private static let TAG = "SocketClient"
    private var urlSession: URLSession?
    private var webSocketTask: URLSessionWebSocketTask?
    private let reconnectDelay: TimeInterval = 5.0
    private let pingDelay: TimeInterval = 10.0
    private var cancellables = Set<AnyCancellable>()
    private let subject = PassthroughSubject<WebSocketState, Never>()
    private let delegateQueue = OperationQueue()
    private var urlString: String = ""
    private var keepConnect = true

    func connect(urlString: String) {
        keepConnect = true
        _ = connectWebSocket(urlString: urlString)
        pingPong()
    }

    private func connectWebSocket(urlString: String) -> Bool {
        do {
            DevKitLogger.debug(SocketClient.TAG, "urlString:" + urlString)
            self.urlString = urlString
            let url = URL(string: urlString)
            let sessionConfig = URLSessionConfiguration.default

            urlSession = URLSession(configuration: sessionConfig, delegate: self, delegateQueue: OperationQueue())
            webSocketTask = urlSession?.webSocketTask(with: url!)
            webSocketTask?.resume()
            listen()
            subject.send(.connecting)

            return true
        } catch {
            self.subject.send(.error(message: error.localizedDescription, error: error))
            if keepConnect {
                reconnect()
            } else {
                subject.send(.disconnected)
            }
            return false
        }
    }

    func disconnect() {
        keepConnect = false
        webSocketTask?.cancel(with: .goingAway, reason: nil)
        subject.send(.disconnected)
    }

    func send(message: SignalingModel) {
        do {
            let encoder = JSONEncoder()
            let data: Data

            // Type-specific encoding
            switch message {
            case let msg as StartSessionMessage:
                data = try encoder.encode(msg)
            case let msg as EndSessionMessage:
                data = try encoder.encode(msg)
            case let msg as HotReloadMessage:
                data = try encoder.encode(msg)
            case let msg as AvailableTargetsMessage:
                data = try encoder.encode(msg)
            case let msg as LogEventMessage:
                data = try encoder.encode(msg)
            default:
                DevKitLogger.debug(SocketClient.TAG, "Unknown message type")
                return
            }

            if let messageString = String(data: data, encoding: .utf8) {
                DevKitLogger.debug(SocketClient.TAG, "Sending => \(messageString)")
                let messageFrame = URLSessionWebSocketTask.Message.string(messageString)
                webSocketTask?.send(messageFrame) { error in
                    if let error = error {
                        DevKitLogger.debug(SocketClient.TAG, "Sending error: \(error.localizedDescription)")
                        self.subject.send(.error(message: error.localizedDescription, error: error))
                    }
                }
            }
        } catch {
            DevKitLogger.debug(SocketClient.TAG, "Sending error: \(error.localizedDescription)")
            subject.send(.error(message: error.localizedDescription, error: error))
        }
    }

    func signals() -> AnyPublisher<WebSocketState, Never> {
        return subject.eraseToAnyPublisher()
    }

    private func listen() {
        webSocketTask?.receive { [weak self] result in
            switch result {
            case .failure(let error):
                DevKitLogger.debug(SocketClient.TAG, "Receive error: \(error.localizedDescription)")
                self?.subject.send(.error(message: error.localizedDescription, error: error))
                self?.reconnect()
            case .success(let message):
                switch message {
                case .string(let text):
                    DevKitLogger.debug(SocketClient.TAG, "Receive <= \(text)")
                    if let signal = self?.deserializeSignal(data: Data(text.utf8)) {
                        self?.subject.send(.receiveSignal(signal))
                    }
                case .data(let data):
                    DevKitLogger.debug(SocketClient.TAG, "Received binary data: \(data)")
                @unknown default:
                    DevKitLogger.debug(SocketClient.TAG, "Received unknown message type")
                }
                self?.listen()
            }
        }
    }

    private struct TypeContainer: Codable {
        let type: SignalingTypeModel
    }

    private func deserializeSignal(data: Data) -> SignalingModel? {
        do {
            let decoder = JSONDecoder()
            let typeContainer = try decoder.decode(TypeContainer.self, from: data)

            switch typeContainer.type {
            case .START_SESSION:
                return try decoder.decode(StartSessionMessage.self, from: data)
            case .END_SESSION:
                return try decoder.decode(EndSessionMessage.self, from: data)
            case .HOT_RELOAD:
                return try decoder.decode(HotReloadMessage.self, from: data)
            case .AVAILABLE_TARGETS:
                return try decoder.decode(AvailableTargetsMessage.self, from: data)
            case .LOG_EVENT:
                return try decoder.decode(LogEventMessage.self, from: data)
            }
        } catch {
            DevKitLogger.debug(SocketClient.TAG, "Failed to deserialize signal: \(error.localizedDescription)")
            return nil
        }
    }

    private func pingPong() {
        DispatchQueue.main.asyncAfter(deadline: .now() + pingDelay) { [weak self] in
            guard let self = self, let webSocketTask = self.webSocketTask else { return }
            webSocketTask.sendPing { (error) in
                if let error = error {
                    DevKitLogger.debug(SocketClient.TAG, "Ping failed: \(error)")
                }
                if self.keepConnect {
                    self.pingPong()
                }
            }
        }
    }

    private func reconnect() {
        DevKitLogger.debug(SocketClient.TAG, "Reconnect in \(reconnectDelay) seconds")
        DispatchQueue.main.asyncAfter(deadline: .now() + reconnectDelay) { [weak self] in
            guard let self = self, let webSocketTask = self.webSocketTask else { return }
            if webSocketTask.state == .completed {
                if keepConnect {
                    _ = self.connectWebSocket(urlString: self.urlString)
                }
            }
        }
    }

    func urlSession(_ session: URLSession, webSocketTask: URLSessionWebSocketTask, didOpenWithProtocol protocol: String?) {
        DevKitLogger.debug(SocketClient.TAG, "State: Connected")
        subject.send(.connected)
    }

    func urlSession(
        _ session: URLSession, webSocketTask: URLSessionWebSocketTask, didCloseWith closeCode: URLSessionWebSocketTask.CloseCode,
        reason: Data?
    ) {
        DevKitLogger.debug(SocketClient.TAG, "State: Disconnected")
        if closeCode == .goingAway {
            subject.send(.disconnected)
        } else {
            subject.send(.connecting)
            reconnect()
        }
    }
}

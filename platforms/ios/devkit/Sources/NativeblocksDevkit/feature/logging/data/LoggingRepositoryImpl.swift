import Foundation
import NativeblocksRuntime

internal class LoggingRepositoryImpl: LoggingRepository {
    private static let TAG = "LoggingRepository"

    private let signalingClient: SocketClient
    private let kitEnvironment: DevKitEnvironment
    private let remoteSession: IRemoteSession

    private var uuid: String {
        remoteSession.getUUID()
    }

    private lazy var projectId: String = {
        JWTUtils.decodedBody(kitEnvironment.apiKey)?["id"] as? String ?? ""
    }()

    init(
        signalingClient: SocketClient,
        kitEnvironment: DevKitEnvironment,
        remoteSession: IRemoteSession
    ) {
        self.signalingClient = signalingClient
        self.kitEnvironment = kitEnvironment
        self.remoteSession = remoteSession
    }

    func sendLog(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String]
    ) async throws {
        do {
            let logMessage = LogEventMessage(
                clientId: uuid,
                roomId: projectId,
                level: level.rawValue,
                event: event,
                message: message,
                parameters: parameters
            )

            signalingClient.send(message: logMessage)
            DevKitLogger.debug(Self.TAG, "Log sent: event=\(event), level=\(level.rawValue)")
        } catch {
            DevKitLogger.debug(Self.TAG, "Failed to send log message: \(error.localizedDescription)")
            throw error
        }
    }
}

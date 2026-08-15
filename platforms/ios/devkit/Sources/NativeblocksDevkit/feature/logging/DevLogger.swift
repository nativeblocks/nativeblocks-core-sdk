import Foundation
import NativeblocksRuntime

internal class DevLogger: INativeLogger {
    private static let TAG = "DevLogger"

    private let loggingRepository: LoggingRepository
    private let queue = DispatchQueue(label: "io.nativeblocks.devkit.devlogger", qos: .utility)

    init(repository: LoggingRepository) {
        self.loggingRepository = repository
    }

    func log(
        level: LoggerEventLevel,
        event: String,
        message: String,
        parameters: [String: String]
    ) {
        DevKitLogger.debug(Self.TAG, "Logging event: \(event), level: \(level.rawValue), message: \(message)")

        queue.async { [weak self] in
            guard let self = self else { return }

            Task {
                do {
                    try await self.loggingRepository.sendLog(
                        level: level,
                        event: event,
                        message: message,
                        parameters: parameters
                    )
                } catch {
                    DevKitLogger.debug(Self.TAG, "Failed to send log: \(error.localizedDescription)")
                }
            }
        }
    }
}

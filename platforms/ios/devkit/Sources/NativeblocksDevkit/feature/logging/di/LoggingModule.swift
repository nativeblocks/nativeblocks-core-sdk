import Foundation

internal class LoggingModule {
    private static var loggingRepository: LoggingRepository?
    private static var devLogger: DevLogger?

    static func provideLoggingRepository() -> LoggingRepository {
        if loggingRepository == nil {
            loggingRepository = LoggingRepositoryImpl(
                signalingClient: LiveModule.proviveSocketClient(),
                kitEnvironment: DevKitInjector.provideEnvironment(),
                remoteSession: DevKitInjector.provideRemoteSession()
            )
        }
        return loggingRepository!
    }

    static func provideDevLogger() -> DevLogger {
        if devLogger == nil {
            devLogger = DevLogger(repository: provideLoggingRepository())
        }
        return devLogger!
    }

    static func destroy() {
        DevKitLogger.debug("LoggingModule", "destroy")
        devLogger = nil
        loggingRepository = nil
    }
}

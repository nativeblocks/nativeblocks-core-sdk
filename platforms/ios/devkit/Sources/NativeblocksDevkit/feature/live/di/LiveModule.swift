import Foundation

internal class LiveModule {
    private static var liveService: LiveService?
    private static var liveRepository: LiveRepository?
    private static var socketClient: SocketClient?

    static func provideLiveRepository() -> LiveRepository {
        if liveRepository == nil {
            liveRepository = LiveRepositoryImpl(
                signalingClient: proviveSocketClient(),
                kitEnvironment: DevKitInjector.provideEnvironment(),
                remoteSession: DevKitInjector.provideRemoteSession()
            )
        }
        return liveRepository!
    }

    static func proviveSocketClient() -> SocketClient {
        if socketClient == nil {
            socketClient = SocketClient()
        }
        return socketClient!
    }

    static func provideLiveViewModel() -> LiveViewModel {
        return LiveViewModel(
            authRepository: AuthModule.provideAuthRepository(),
            liveRepository: LiveModule.provideLiveRepository(),
            kitEnvironment: DevKitInjector.provideEnvironment()
        )
    }

    static func provideLiveService() -> LiveService {
        if liveService == nil {
            liveService = LiveService(
                viewModel: LiveModule.provideLiveViewModel()
            )
        }
        return liveService!
    }

    static func destroy() {
        DevKitLogger.debug("LiveModule", "destroy")
        liveService?.cleanUp()
        liveService = nil

        liveRepository?.cleanUp()
        liveRepository = nil

        socketClient = nil
    }
}

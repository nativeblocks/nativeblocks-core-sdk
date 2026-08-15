import Combine
import Foundation
import NativeblocksRuntime

internal class LiveViewModel: ObservableObject {
    @Published private(set) var uiState: LiveState = .initial()

    private let authRepository: AuthRepository
    private let liveRepository: LiveRepository
    private let kitEnvironment: DevKitEnvironment
    var cancellables: Set<AnyCancellable> = []

    init(authRepository: AuthRepository, liveRepository: LiveRepository, kitEnvironment: DevKitEnvironment) {
        self.authRepository = authRepository
        self.liveRepository = liveRepository
        self.kitEnvironment = kitEnvironment

        liveRepository.orientedChange()

        authRepository.token()
            .sink { token in
                if !token.isEmpty {
                    if self.uiState.notificationState != .connect && self.uiState.notificationState != .connecting {
                        if kitEnvironment.autoConnect {
                            self.startStream()
                        } else {
                            self.uiState.notificationState = .notConnect
                        }
                    }
                }
            }
            .store(in: &cancellables)

        if authRepository.authCheck() {
            uiState.notificationState = .notConnect
        } else {
            uiState.notificationState = .authRequire
        }

        liveRepository.connectionState()
            .sink { state in
                if authRepository.authCheck() {
                    switch state {
                    case .connected:
                        self.uiState.notificationState = .connect
                    case .connecting:
                        self.uiState.notificationState = .connecting
                    case .notConnected:
                        self.uiState.notificationState = .notConnect
                    }
                } else {
                    self.uiState.notificationState = .authRequire
                }
            }
            .store(in: &cancellables)

        liveRepository.hotReload()
            .sink { route in
                Task {
                    DevKitLogger.debug("LiveViewModel", "HotReload \(route)")
                    if !route.isEmpty {
                        _ = await NativeblocksManager.getInstance(name: kitEnvironment.instanceName).syncFrame(route: route)
                    }
                }
            }
            .store(in: &cancellables)
    }

    private func startStream() {
        if kitEnvironment.screenSharing {
            // NO-OP -> presentScreenSharing()
        }
        liveRepository.start()
    }

    private func endStream() {
        liveRepository.end()
    }

    func updateAction(state: Action) {
        switch state {
        case .connect:
            if authRepository.authCheck() {
                startStream()
            } else {
                uiState.notificationState = .authRequire
            }
        case .disconnect:
            endStream()
        case .start:
            if uiState.notificationState != .connect && uiState.notificationState != .connecting {
                if authRepository.authCheck() {
                    if kitEnvironment.autoConnect {
                        startStream()
                    } else {
                        uiState.notificationState = .notConnect
                    }
                } else {
                    uiState.notificationState = .authRequire
                }
            }
        case .auth:
            AuthScreenSheet.present {
                self.uiState.notificationState = self.uiState.notificationState
            }
        case .logout:
            endStream()
            authRepository.logout()
            AuthScreenSheet.present {
                self.uiState.notificationState = .authRequire
            }
        }
    }
}

import Foundation

internal class AuthModule {

    static func provideAuthRepository() -> AuthRepository {
        return AuthRepositoryImpl(cache: DevKitInjector.provideCache())
    }

    static func provideAuthViewModel() -> AuthViewModel {
        return AuthViewModel(authRepository: provideAuthRepository())
    }

    static func destroy() {
        DevKitLogger.debug("AuthModule", "destroy")
    }
}

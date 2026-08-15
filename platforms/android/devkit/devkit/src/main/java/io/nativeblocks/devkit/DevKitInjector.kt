package io.nativeblocks.devkit

import android.content.Context
import io.nativeblocks.devkit.feature.auth.di.AuthModule
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.feature.live.di.LiveModule
import io.nativeblocks.devkit.feature.live.presenter.LiveService
import io.nativeblocks.devkit.feature.logging.di.LoggingModule
import io.nativeblocks.devkit.lib.cache.CacheModule
import io.nativeblocks.devkit.session.IRemoteSession
import io.nativeblocks.devkit.session.RemoteSessionImpl
import io.nativeblocks.devkit.session.DevKitEnvironment
import org.koin.android.ext.koin.androidContext
import org.koin.core.KoinApplication
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.singleOf
import org.koin.dsl.koinApplication
import org.koin.dsl.module

internal object DevKitInjector {
    private var koinApp: KoinApplication? = null

    fun get(): KoinApplication {
        if (koinApp == null) {
            throw NullPointerException("Please make sure the init function has been called before get")
        }
        return koinApp!!
    }

    fun init(context: Context, environment: DevKitEnvironment) {
        if (koinApp == null) {
            synchronized(DevKitInjector::class.java) {
                if (koinApp == null) {
                    koinApp = koinApplication {
                        androidContext(context.applicationContext)
                        modules(
                            module {
                                single<DevKitEnvironment> { environment }
                                singleOf(::RemoteSessionImpl) { bind<IRemoteSession>() }
                            },
                            CacheModule,
                            AuthModule,
                            LiveModule,
                            LoggingModule,
                        )
                    }
                }
            }
        }
    }

    fun destroy() {
        val context: Context? = koinApp?.koin?.get()
        context?.let {
            LiveService.stop(context = context)
        }
        koinApp?.close()
        koinApp = null
    }

    suspend fun userLogout() {
        val authRepository: AuthRepository = get().koin.get()
        authRepository.logout()
    }

    suspend fun userAuthorization(endpoint: String, token: String, realtimeEndpoint: String) {
        val authRepository: AuthRepository = get().koin.get()
        authRepository.updateTokenAndEndpoint(
            endpoint = endpoint,
            token = token,
            realtimeEndpoint = realtimeEndpoint
        )
    }
}
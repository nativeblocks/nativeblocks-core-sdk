package io.nativeblocks.devkit.feature.live.di

import io.ktor.client.HttpClient
import io.ktor.client.engine.okhttp.OkHttp
import io.ktor.client.plugins.websocket.WebSockets
import io.nativeblocks.devkit.feature.auth.data.AuthRepositoryImpl
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.feature.live.data.LiveRepositoryImpl
import io.nativeblocks.devkit.feature.live.data.SocketClient
import io.nativeblocks.devkit.feature.live.domain.repository.LiveRepository
import io.nativeblocks.devkit.feature.live.presenter.LiveViewModel
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.factoryOf
import org.koin.core.module.dsl.viewModel
import org.koin.dsl.module

internal val LiveModule = module {
    single {
        val client = HttpClient(OkHttp) {
            install(WebSockets) {
                pingInterval = 20_000L
            }
        }
        SocketClient(client = client)
    }
    factoryOf(::LiveRepositoryImpl) { bind<LiveRepository>() }
    factoryOf(::AuthRepositoryImpl) { bind<AuthRepository>() }
    viewModel { LiveViewModel(get(), get(), get()) }
}
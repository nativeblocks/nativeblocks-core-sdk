package io.nativeblocks.runtime.di

import android.content.Context
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.engine.CacheProvider
import io.nativeblocks.runtime.engine.HttpClient
import io.nativeblocks.runtime.engine.NativeEngineClientManager
import io.nativeblocks.runtime.experiment.ExperimentUseCase
import io.nativeblocks.runtime.frame.FrameStateBridgeImpl
import io.nativeblocks.runtime.frame.FrameViewModel
import io.nativeblocks.runtime.lib.NativeSqliteHelper
import io.nativeblocks.runtime.lib.OkHttpHttpClient
import io.nativeblocks.runtime.lib.SqliteCacheProvider
import io.nativeblocks.runtime.localization.LocalizationUseCase
import okhttp3.OkHttpClient
import org.koin.android.ext.koin.androidContext
import org.koin.core.KoinApplication
import org.koin.core.module.Module
import org.koin.core.module.dsl.viewModel
import org.koin.core.qualifier.named
import org.koin.dsl.koinApplication
import org.koin.dsl.module
import java.util.concurrent.TimeUnit

internal object NativeCoreSDKInjector {
    private val injectorRegistry = mutableMapOf<String, KoinApplication>()

    fun get(name: String): KoinApplication {
        return injectorRegistry[name]
            ?: throw NullPointerException("Please make sure the '$name' init function has been called before get")
    }

    fun init(name: String, context: Context, edition: NativeblocksEdition) {
        synchronized(this) {
            if (!injectorRegistry.containsKey(name)) {
                val app = koinApplication {
                    androidContext(context)
                    modules(
                        engineModule(name, edition),
                        featureModule(name),
                    )
                }
                injectorRegistry[name] = app
            }
        }
    }

    fun destroy(name: String) {
        injectorRegistry[name]?.close()
        injectorRegistry.remove(name)
    }
}

internal fun featureModule(instanceName: String): Module {
    return module {
        viewModel(named(instanceName)) {
            val engine = get<NativeEngineClientManager>(named(instanceName))
            FrameViewModel(
                frameStateBridge = FrameStateBridgeImpl(engine.frameStateManager()),
                instanceName = instanceName,
            )
        }

        single(named(instanceName)) {
            ExperimentUseCase(get(named(instanceName)))
        }

        single(named(instanceName)) {
            val engine = get<NativeEngineClientManager>(named(instanceName))
            LocalizationUseCase(
                engine,
                engine.localizationStateManager()
            )
        }

    }
}

internal fun engineModule(instanceName: String, edition: NativeblocksEdition) = module {
    single<CacheProvider>(named(instanceName)) {
        SqliteCacheProvider(
            NativeSqliteHelper(
                androidContext(),
                "NATIVEBLOCKS_DATABASE_$instanceName"
            ),
        )
    }

    single(named(instanceName)) {
        OkHttpClient.Builder()
            .connectTimeout(10, TimeUnit.SECONDS)
            .readTimeout(10, TimeUnit.SECONDS)
            .build()
    }

    single<HttpClient>(named(instanceName), createdAtStart = false) {
        OkHttpHttpClient(get(named(instanceName)))
    }

    single(named(instanceName)) {
        NativeEngineClientManager(
            instanceName = instanceName,
            edition = edition,
            http = get(named(instanceName)),
            cache = get(named(instanceName)),
        )
    }
}
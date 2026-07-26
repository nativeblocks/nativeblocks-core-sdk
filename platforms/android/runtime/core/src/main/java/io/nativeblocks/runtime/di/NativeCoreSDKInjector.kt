package io.nativeblocks.runtime.di

import android.content.Context
import io.nativeblocks.runtime.api.NativeblocksEdition
import io.nativeblocks.runtime.ffi.CacheProvider
import io.nativeblocks.runtime.ffi.HttpClient
import io.nativeblocks.runtime.ffi.NativeRuntimeClientManager
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
                        ffiModule(name, edition),
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
            val runtimeClientManager = get<NativeRuntimeClientManager>(named(instanceName))
            FrameViewModel(
                frameStateBridge = FrameStateBridgeImpl(runtimeClientManager.frameStateManager()),
                instanceName = instanceName,
            )
        }

        single(named(instanceName)) {
            ExperimentUseCase(get(named(instanceName)))
        }

        single(named(instanceName)) {
            val runtimeClientManager = get<NativeRuntimeClientManager>(named(instanceName))
            LocalizationUseCase(
                runtimeClientManager,
                runtimeClientManager.localizationStateManager()
            )
        }

    }
}

internal fun ffiModule(instanceName: String, edition: NativeblocksEdition) = module {
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
        NativeRuntimeClientManager(
            instanceName = instanceName,
            edition = edition,
            http = get(named(instanceName)),
            cache = get(named(instanceName)),
        )
    }
}
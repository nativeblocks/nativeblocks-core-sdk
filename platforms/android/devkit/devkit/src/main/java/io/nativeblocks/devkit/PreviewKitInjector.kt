package io.nativeblocks.devkit

import android.content.Context
import io.nativeblocks.devkit.feature.preview.di.PreviewModule
import io.nativeblocks.devkit.session.PreviewKitEnvironment
import org.koin.android.ext.koin.androidContext
import org.koin.core.KoinApplication
import org.koin.dsl.koinApplication
import org.koin.dsl.module

internal object PreviewKitInjector {
    private var koinApp: KoinApplication? = null

    fun get(): KoinApplication {
        if (koinApp == null) {
            throw NullPointerException("Please make sure the init function has been called before get")
        }
        return koinApp!!
    }

    fun init(context: Context, environment: PreviewKitEnvironment) {
        if (koinApp == null) {
            synchronized(PreviewKitInjector::class.java) {
                if (koinApp == null) {
                    koinApp = koinApplication {
                        androidContext(context.applicationContext)
                        modules(
                            module {
                                single<PreviewKitEnvironment> { environment }
                            },
                            PreviewModule,
                        )
                    }
                }
            }
        }
    }

    fun destroy() {
        koinApp?.close()
        koinApp = null
    }
}
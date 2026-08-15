package io.nativeblocks.devkit.lib.cache

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.preferencesDataStore
import org.koin.android.ext.koin.androidContext
import org.koin.core.module.dsl.bind
import org.koin.core.module.dsl.singleOf
import org.koin.dsl.module

private val Context.dataStore by preferencesDataStore(name = "DEVKIT")

internal val CacheModule = module {
    single<DataStore<Preferences>> { androidContext().dataStore }
    singleOf(::NativeCacheStorage) { bind<INativeCache>() }
}
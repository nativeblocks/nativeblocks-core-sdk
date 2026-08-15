package io.nativeblocks.devkit.lib.cache

import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.doublePreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.floatPreferencesKey
import androidx.datastore.preferences.core.intPreferencesKey
import androidx.datastore.preferences.core.longPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.runBlocking

internal class NativeCacheStorage(private val dataStore: DataStore<Preferences>) : INativeCache {

    override fun push(key: String, value: String?) {
        runBlocking {
            dataStore.edit { settings ->
                settings[stringPreferencesKey(key)] = value.orEmpty()
            }
        }
    }

    override fun push(key: String, value: Boolean) {
        runBlocking {
            dataStore.edit { settings ->
                settings[booleanPreferencesKey(key)] = value
            }
        }
    }

    override fun push(key: String, value: Float) {
        runBlocking {
            dataStore.edit { settings ->
                settings[floatPreferencesKey(key)] = value
            }
        }
    }

    override fun push(key: String, value: Double) {
        runBlocking {
            dataStore.edit { settings ->
                settings[doublePreferencesKey(key)] = value
            }
        }
    }

    override fun push(key: String, value: Int) {
        runBlocking {
            dataStore.edit { settings ->
                settings[intPreferencesKey(key)] = value
            }
        }
    }

    override fun push(key: String, value: Long) {
        runBlocking {
            dataStore.edit { settings ->
                settings[longPreferencesKey(key)] = value
            }
        }
    }

    override fun pull(key: String, defaultValue: String): String {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[stringPreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun pull(key: String, defaultValue: Boolean): Boolean {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[booleanPreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun pull(key: String, defaultValue: Float): Float {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[floatPreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun pull(key: String, defaultValue: Double): Double {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[doublePreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun pull(key: String, defaultValue: Int): Int {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[intPreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun pull(key: String, defaultValue: Long): Long {
        return runBlocking {
            dataStore.data.map { prefs -> prefs[longPreferencesKey(key)] }.first() ?: defaultValue
        }
    }

    override fun clear() {
        runBlocking {
            dataStore.edit { settings ->
                settings.clear()
            }
        }
    }

    override fun delete(key: String) {
        runBlocking {
            dataStore.edit { settings ->
                settings.remove(stringPreferencesKey(key))
            }
        }
    }

    override fun onValueChange(key: String, defaultValue: String): Flow<String> {
        return dataStore.data.map { pref -> pref[stringPreferencesKey(key)] ?: defaultValue }
    }

    override fun onValueChange(key: String, defaultValue: Boolean): Flow<Boolean> {
        return dataStore.data.map { pref -> pref[booleanPreferencesKey(key)] ?: defaultValue }
    }

    override fun onValueChange(key: String, defaultValue: Int): Flow<Int> {
        return dataStore.data.map { pref -> pref[intPreferencesKey(key)] ?: defaultValue }
    }

    override fun onValueChange(key: String, defaultValue: Long): Flow<Long> {
        return dataStore.data.map { pref -> pref[longPreferencesKey(key)] ?: defaultValue }
    }

    override fun onValueChange(key: String, defaultValue: Float): Flow<Float> {
        return dataStore.data.map { pref -> pref[floatPreferencesKey(key)] ?: defaultValue }
    }

    override fun onValueChange(key: String, defaultValue: Double): Flow<Double> {
        return dataStore.data.map { pref -> pref[doublePreferencesKey(key)] ?: defaultValue }
    }

}
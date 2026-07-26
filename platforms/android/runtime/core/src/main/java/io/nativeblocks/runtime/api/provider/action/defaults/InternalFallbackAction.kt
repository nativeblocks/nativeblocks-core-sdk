package io.nativeblocks.runtime.api.provider.action.defaults

import android.content.Context
import android.widget.Toast
import io.nativeblocks.runtime.di.NativeCoreSDKInjector

internal fun internalFallbackAction(instanceName: String, name: String) {
    val koin = NativeCoreSDKInjector.get(instanceName).koin
    val context: Context = koin.get()
    Toast.makeText(
        context,
        "The $name action isn’t available in this app version",
        Toast.LENGTH_LONG
    ).show()
}
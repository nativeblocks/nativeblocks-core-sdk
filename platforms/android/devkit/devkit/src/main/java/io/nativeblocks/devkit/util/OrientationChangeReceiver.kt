package io.nativeblocks.devkit.util

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.callbackFlow

internal fun registerOrientationChange(context: Context) = callbackFlow {
    val orientationReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context?, intent: Intent?) {
            val orientation = context?.resources?.configuration?.orientation
            trySend(orientation ?: 0)
        }
    }

    try {
        val filter = IntentFilter(Intent.ACTION_CONFIGURATION_CHANGED)
        context.registerReceiver(orientationReceiver, filter)
    } catch (e: Exception) {
        close(e)
    }

    awaitClose {
        try {
            context.unregisterReceiver(orientationReceiver)
        } catch (_: Exception) {
        }
    }
}
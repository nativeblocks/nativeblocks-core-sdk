package io.nativeblocks.devkit.util

import android.util.Log

internal object DevKitLogger {
    private const val isDebuggable = false
    fun d(tag: String?, msg: String) {
        if (isDebuggable)
            Log.d(tag, msg)
    }

    fun d(tag: String?, msg: String?, tr: Throwable?) {
        if (isDebuggable)
            Log.d(tag, msg, tr)
    }

    fun e(tag: String?, msg: String) {
        if (isDebuggable)
            Log.e(tag, msg)
    }

    fun e(tag: String?, msg: String?, tr: Throwable?) {
        if (isDebuggable)
            Log.e(tag, msg, tr)
    }
}
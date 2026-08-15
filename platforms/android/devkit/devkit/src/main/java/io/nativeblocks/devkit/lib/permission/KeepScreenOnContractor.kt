package io.nativeblocks.devkit.lib.permission

import android.app.Activity
import android.view.WindowManager
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.ui.platform.LocalContext
import io.nativeblocks.runtime.api.provider.action.INativeActionContractor

internal class KeepScreenOnContractor : INativeActionContractor {
    @Composable
    override fun ActionContractor() {
        val context = LocalContext.current
        val activity = context as? Activity
        DisposableEffect(Unit) {
            activity?.window?.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            onDispose {
                activity?.window?.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
            }
        }
    }
}
package io.nativeblocks.devkit.lib.permission

import android.content.pm.PackageManager.PERMISSION_GRANTED
import androidx.activity.ComponentActivity
import androidx.activity.result.ActivityResultCallback
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContract
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import java.util.UUID

private fun <I, O> ComponentActivity.registerActivityResultLauncher(
    contract: ActivityResultContract<I, O>,
    callback: ActivityResultCallback<O>
): ActivityResultLauncher<I> {
    val key = UUID.randomUUID().toString()
    return activityResultRegistry.register(key, contract, callback)
}

internal class NotificationPermissionContractor(
    private val activity: ComponentActivity
) {
    private var permissionLauncher: ActivityResultLauncher<Array<String>>? = null

    fun isPermissionRequired(): Boolean {
        val permissions = PermissionType.Notification.permissions
        if (permissions.isEmpty()) return false

        return permissions.any { permission ->
            ContextCompat.checkSelfPermission(activity, permission) != PERMISSION_GRANTED
        }
    }

    fun requestPermission(onResult: (Boolean) -> Unit) {
        val permissions = PermissionType.Notification.permissions
        if (permissions.isEmpty()) {
            onResult(true)
            return
        }

        permissionLauncher = activity.registerActivityResultLauncher(
            contract = ActivityResultContracts.RequestMultiplePermissions(),
            callback = { results ->
                val allGranted = results.values.all { it }
                onResult(allGranted)
                permissionLauncher?.unregister()
                permissionLauncher = null
            }
        )
        permissionLauncher?.launch(permissions.toList().toTypedArray())
    }

    fun unregister() {
        permissionLauncher?.unregister()
        permissionLauncher = null
    }
}

package io.nativeblocks.devkit.lib.permission

import android.Manifest.permission.CAMERA
import android.Manifest.permission.POST_NOTIFICATIONS
import android.os.Build

internal data class PermissionResult(
    val name: String,
    val isGranted: Boolean
)

private val notificationPermission: Array<String> =
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) arrayOf(POST_NOTIFICATIONS) else arrayOf()

internal enum class PermissionType(vararg val permissions: String) {
    Camera(CAMERA),
    Notification(permissions = notificationPermission);

    operator fun plus(permissionType: PermissionType) =
        this.permissions.toList() + permissionType.permissions.toList()
}

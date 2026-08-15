package io.nativeblocks.devkit.lib.permission

import android.content.Context
import android.content.pm.PackageManager
import androidx.activity.compose.ManagedActivityResultLauncher
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.runtime.Composable
import androidx.core.content.ContextCompat
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.update

internal class PermissionContractor(private val context: Context) {

    val permissionsRequested = MutableStateFlow<List<PermissionResult>>(emptyList())
    val permissionsCheck = MutableStateFlow<List<PermissionResult>>(emptyList())
    private var launcher: ManagedActivityResultLauncher<Array<String>, Map<String, Boolean>>? = null

    fun sendRequest(permissions: List<String>) {
        launcher?.launch(permissions.toTypedArray())
    }

    fun checkPermission(permissions: List<String>) {
        permissionsCheck.update {
            permissions.map { perm ->
                PermissionResult(
                    name = perm,
                    isGranted = ContextCompat.checkSelfPermission(
                        context,
                        perm
                    ) == PackageManager.PERMISSION_GRANTED
                )
            }
        }
    }

    @Composable
    fun PermissionContractorCollector() {
        launcher =
            rememberLauncherForActivityResult(ActivityResultContracts.RequestMultiplePermissions()) { requests ->
                val permissions = mutableListOf<PermissionResult>()
                requests.forEach { rq ->
                    permissions.add(PermissionResult(name = rq.key, isGranted = rq.value))
                }
                permissionsRequested.update { permissions }
            }
    }
}
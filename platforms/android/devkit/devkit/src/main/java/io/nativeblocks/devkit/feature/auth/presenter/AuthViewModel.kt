package io.nativeblocks.devkit.feature.auth.presenter

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.nativeblocks.devkit.lib.permission.PermissionContractor
import io.nativeblocks.devkit.lib.permission.PermissionType
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.feature.auth.presenter.AuthContract.UIAction
import io.nativeblocks.devkit.feature.auth.presenter.AuthContract.UIEffect
import io.nativeblocks.devkit.feature.auth.presenter.AuthContract.UIState
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

internal class AuthViewModel(
    private val authRepository: AuthRepository,
    private val permissionContractor: PermissionContractor
) : ViewModel() {

    private val _uiState = MutableStateFlow(UIState.init())
    val uiState: StateFlow<UIState> = _uiState.asStateFlow()

    private val _uiEffect = Channel<UIEffect>(Channel.BUFFERED)
    val uiEffect: Flow<UIEffect> = _uiEffect.receiveAsFlow()

    init {
        permissionObserver()
        permissionContractor.checkPermission(PermissionType.Camera.permissions.toList())
    }

    fun onAction(uiAction: UIAction) {
        viewModelScope.launch {
            when (uiAction) {
                is UIAction.OnScan -> {
                    if (authRepository.updateTokenAndEndpoint(uiAction.qrData)) {
                        sendEffect(UIEffect.Close)
                    } else {
                        sendEffect(UIEffect.ShowToast("Could not read the QR, please try again"))
                    }
                }

                UIAction.OnPermissionRequest -> {
                    permissionContractor.sendRequest(PermissionType.Camera.permissions.toList())
                }

                UIAction.OnClose -> {
                    sendEffect(UIEffect.Close)
                }

                UIAction.CopyFromClipboard -> {
                    _uiState.update { it.copy(pasteFromClipboard = true) }
                }

                is UIAction.OnCopyFromClipboardFinished -> {
                    _uiState.update { it.copy(pasteFromClipboard = false) }
                    if (uiAction.clipboard != null) {
                        sendEffect(UIEffect.ShowToast("Copy from clipboard"))
                        if (authRepository.updateTokenAndEndpoint(uiAction.clipboard)) {
                            sendEffect(UIEffect.Close)
                        } else {
                            sendEffect(UIEffect.ShowToast("Could not read the clipboard, Auth its invalid, please try again"))
                        }
                    } else {
                        sendEffect(UIEffect.ShowToast("Could not read the clipboard, please try again"))
                    }
                }
            }
        }
    }

    private suspend fun sendEffect(uiEffect: UIEffect) {
        _uiEffect.send(uiEffect)
    }

    private fun permissionObserver() {
        viewModelScope.launch {
            permissionContractor.permissionsCheck.collect { result ->
                result.forEach { permission ->
                    _uiState.update { it.copy(hasPermission = permission.isGranted) }
                }
            }
        }
        viewModelScope.launch {
            permissionContractor.permissionsRequested.collect { result ->
                result.forEach { permission ->
                    _uiState.update { it.copy(hasPermission = permission.isGranted) }
                }
            }
        }
    }
}

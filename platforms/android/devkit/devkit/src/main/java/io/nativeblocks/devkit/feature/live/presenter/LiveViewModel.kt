package io.nativeblocks.devkit.feature.live.presenter

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.nativeblocks.devkit.feature.auth.domain.repository.AuthRepository
import io.nativeblocks.devkit.feature.live.domain.model.ConnectionState
import io.nativeblocks.devkit.feature.live.domain.repository.LiveRepository
import io.nativeblocks.devkit.session.DevKitEnvironment
import io.nativeblocks.runtime.api.NativeblocksManager
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

internal class LiveViewModel(
    private val authRepository: AuthRepository,
    private val liveRepository: LiveRepository,
    private val environment: DevKitEnvironment,
) : ViewModel() {

    private val _uiState = MutableStateFlow(LiveState.initial())
    val uiState: Flow<LiveState> = _uiState.asStateFlow()

    init {
        viewModelScope.launch {
            liveRepository.orientedChange()
        }

        viewModelScope.launch {
            authRepository.token().collect { token ->
                if (token.isNotEmpty()) {
                    if (
                        _uiState.value.notificationState != NotificationState.CONNECT
                        && _uiState.value.notificationState != NotificationState.CONNECTING
                    ) {
                        if (environment.autoConnect) {
                            liveRepository.start()
                        } else {
                            _uiState.update {
                                it.copy(notificationState = NotificationState.NOT_CONNECT)
                            }
                        }
                    }
                }
            }
            if (authRepository.authCheck()) {
                _uiState.update {
                    it.copy(notificationState = NotificationState.NOT_CONNECT)
                }
            }
        }

        viewModelScope.launch {
            liveRepository.connectionState().collect { state ->
                when (state) {
                    ConnectionState.Connected -> {
                        _uiState.update {
                            it.copy(notificationState = NotificationState.CONNECT)
                        }
                    }

                    ConnectionState.Connecting -> {
                        _uiState.update {
                            it.copy(notificationState = NotificationState.CONNECTING)
                        }
                    }

                    ConnectionState.NotConnected -> {
                        _uiState.update {
                            it.copy(notificationState = NotificationState.NOT_CONNECT)
                        }
                    }
                }
            }
        }

        viewModelScope.launch {
            liveRepository.hotReload().collect { frameRoute ->
                NativeblocksManager.getInstance(environment.instanceName).syncFrame(route = frameRoute)
            }
        }
    }

    fun updateAction(state: Action) {
        viewModelScope.launch {
            when (state) {
                Action.CONNECT -> {
                    liveRepository.start()
                }

                Action.DISCONNECT -> {
                    liveRepository.end()
                }

                Action.START -> {
                    if (
                        _uiState.value.notificationState != NotificationState.CONNECT
                        && _uiState.value.notificationState != NotificationState.CONNECTING
                    ) {
                        if (authRepository.authCheck()) {
                            if (environment.autoConnect) {
                                liveRepository.start()
                            } else {
                                _uiState.update {
                                    it.copy(notificationState = NotificationState.NOT_CONNECT)
                                }
                            }
                        } else {
                            _uiState.update {
                                it.copy(notificationState = NotificationState.AUTH_REQUIRE)
                            }
                        }
                    }
                }

                Action.LOGOUT -> {
                    liveRepository.end()
                    authRepository.logout()
                    _uiState.update {
                        it.copy(notificationState = NotificationState.AUTH_REQUIRE)
                    }
                }
            }
        }
    }
}
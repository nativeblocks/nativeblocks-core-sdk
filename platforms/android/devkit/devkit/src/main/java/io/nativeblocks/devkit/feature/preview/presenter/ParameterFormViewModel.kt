package io.nativeblocks.devkit.feature.preview.presenter

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.nativeblocks.devkit.feature.preview.domain.repository.ParameterRepository
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormContract.UIAction
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormContract.UIEffect
import io.nativeblocks.devkit.feature.preview.presenter.ParameterFormContract.UIState
import io.nativeblocks.runtime.api.NativeblocksManager
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.receiveAsFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

internal class ParameterFormViewModel(
    private val parameterRepository: ParameterRepository,
    private val instanceName: String
) : ViewModel() {

    private val _uiState = MutableStateFlow(UIState.init())
    val uiState: StateFlow<UIState> = _uiState.asStateFlow()

    private val _uiEffect = Channel<UIEffect>(Channel.BUFFERED)
    val uiEffect: Flow<UIEffect> = _uiEffect.receiveAsFlow()

    init {
        loadParameters()
    }

    fun onAction(uiAction: UIAction) {
        when (uiAction) {
            UIAction.LoadParameters -> loadParameters()

            UIAction.StartAddParameter -> {
                _uiState.update { it.copy(isAddingNew = true, newKey = "", newValue = "") }
            }

            is UIAction.UpdateNewKey -> {
                _uiState.update { it.copy(newKey = uiAction.key) }
            }

            is UIAction.UpdateNewValue -> {
                _uiState.update { it.copy(newValue = uiAction.value) }
            }

            UIAction.ConfirmAddParameter -> {
                val state = _uiState.value
                if (state.newKey.isNotBlank()) {
                    parameterRepository.add(state.newKey, state.newValue)
                    _uiState.update {
                        it.copy(
                            isAddingNew = false,
                            newKey = "",
                            newValue = "",
                            parameters = parameterRepository.getAll()
                        )
                    }
                } else {
                    sendEffect(UIEffect.ShowToast("Key cannot be empty"))
                }
            }

            UIAction.CancelAddParameter -> {
                _uiState.update { it.copy(isAddingNew = false, newKey = "", newValue = "") }
            }

            is UIAction.StartEditParameter -> {
                _uiState.update { it.copy(editingParameter = uiAction.parameter) }
            }

            is UIAction.UpdateEditKey -> {
                _uiState.update {
                    it.copy(editingParameter = it.editingParameter?.copy(key = uiAction.key))
                }
            }

            is UIAction.UpdateEditValue -> {
                _uiState.update {
                    it.copy(editingParameter = it.editingParameter?.copy(value = uiAction.value))
                }
            }

            UIAction.ConfirmEditParameter -> {
                val editing = _uiState.value.editingParameter
                if (editing != null && editing.key.isNotBlank()) {
                    parameterRepository.update(editing.id, editing.key, editing.value)
                    _uiState.update {
                        it.copy(
                            editingParameter = null,
                            parameters = parameterRepository.getAll()
                        )
                    }
                }
            }

            UIAction.CancelEditParameter -> {
                _uiState.update { it.copy(editingParameter = null) }
            }

            is UIAction.DeleteParameter -> {
                parameterRepository.delete(uiAction.id)
                _uiState.update { it.copy(parameters = parameterRepository.getAll()) }
            }

            UIAction.ApplyParameters -> {
                val params = parameterRepository.toMap()
                NativeblocksManager.getInstance(instanceName).setGlobalParameters(params)
                sendEffect(UIEffect.ShowToast("Parameters applied"))
                sendEffect(UIEffect.ParametersApplied)
            }

            UIAction.ClearAll -> {
                parameterRepository.clear()
                NativeblocksManager.getInstance(instanceName).setGlobalParameters(emptyMap())
                _uiState.update { it.copy(parameters = emptyList()) }
                sendEffect(UIEffect.ShowToast("All parameters cleared"))
            }

            UIAction.Close -> {
                sendEffect(UIEffect.Dismissed)
            }
        }
    }

    private fun sendEffect(uiEffect: UIEffect) {
        viewModelScope.launch { _uiEffect.send(uiEffect) }
    }

    private fun loadParameters() {
        _uiState.update { it.copy(parameters = parameterRepository.getAll()) }
    }
}

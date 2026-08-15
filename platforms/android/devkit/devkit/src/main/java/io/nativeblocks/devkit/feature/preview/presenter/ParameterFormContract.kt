package io.nativeblocks.devkit.feature.preview.presenter

import io.nativeblocks.devkit.feature.preview.domain.model.Parameter

internal object ParameterFormContract {

    data class UIState(
        val parameters: List<Parameter>,
        val editingParameter: Parameter?,
        val isAddingNew: Boolean,
        val newKey: String,
        val newValue: String
    ) {
        companion object {
            fun init() = UIState(
                parameters = emptyList(),
                editingParameter = null,
                isAddingNew = false,
                newKey = "",
                newValue = ""
            )
        }
    }

    sealed interface UIAction {
        data object LoadParameters : UIAction
        data object StartAddParameter : UIAction
        data class UpdateNewKey(val key: String) : UIAction
        data class UpdateNewValue(val value: String) : UIAction
        data object ConfirmAddParameter : UIAction
        data object CancelAddParameter : UIAction
        data class StartEditParameter(val parameter: Parameter) : UIAction
        data class UpdateEditKey(val key: String) : UIAction
        data class UpdateEditValue(val value: String) : UIAction
        data object ConfirmEditParameter : UIAction
        data object CancelEditParameter : UIAction
        data class DeleteParameter(val id: String) : UIAction
        data object ApplyParameters : UIAction
        data object ClearAll : UIAction
        data object Close : UIAction
    }

    sealed interface UIEffect {
        data class ShowToast(val message: String) : UIEffect
        data object ParametersApplied : UIEffect
        data object Dismissed : UIEffect
    }
}

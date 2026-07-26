package io.nativeblocks.runtime.api.provider.action

import androidx.compose.runtime.Composable
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import kotlinx.coroutines.CoroutineScope

/**
 * Defines the contract for handling native actions within the framework.
 */
interface INativeAction {

    /**
     * Handles the specified action properties.
     * @param actionProps The properties of the action to handle.
     */
    fun handle(actionProps: ActionProps)
}

/**
 * Defines a contract for composable action contractors in the native framework.
 */
interface INativeActionContractor {

    /**
     * Composable function to define the UI for an action contractor.
     */
    @Composable
    fun ActionContractor()

}

/**
 * Represents the properties required to handle an action in the native framework.
 * @param instanceName instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item associated with the action.
 * @param coroutineScope Coroutine scope for executing asynchronous operations.
 * @property onFindVariable Lambda function for retrieving a [NativeVariableModel] by its key.
 * @param onChangeVariable Callback invoked when a variable changes.
 * @param onFindBlock Lambda function for retrieving a [NativeBlockModel] by its identifier.
 * @param onChangeBlockProperties Callback invoked when a block changes.
 * @param trigger Trigger model representing the conditions and outcomes of the action.
 * @param onHandleNextTrigger Callback invoked to handle the next trigger in the sequence.
 * @param onHandleSuccessNextTrigger Callback invoked to handle the next trigger upon success.
 * @param onHandleFailureNextTrigger Callback invoked to handle the next trigger upon failure.
 */
data class ActionProps(
    val instanceName: String,
    val listItemIndex: Int,
    val coroutineScope: CoroutineScope,
    val onFindVariable: (String) -> NativeVariableModel?,
    val onChangeVariable: (NativeVariableModel?) -> Unit,
    val onFindBlock: (String) -> NativeBlockModel?,
    val onChangeBlockProperties: (String, String, String, String, String) -> Unit,
    val trigger: NativeActionTriggerModel?,
    val onHandleNextTrigger: (NativeActionTriggerModel) -> Unit,
    val onHandleSuccessNextTrigger: (NativeActionTriggerModel) -> Unit,
    val onHandleFailureNextTrigger: (NativeActionTriggerModel) -> Unit
)

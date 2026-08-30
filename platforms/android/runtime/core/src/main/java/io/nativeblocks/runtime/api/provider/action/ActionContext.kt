package io.nativeblocks.runtime.api.provider.action

import androidx.compose.runtime.Composable
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import kotlinx.coroutines.CoroutineScope

/**
 * Defines the contract for handling native actions within the framework.
 */
interface INativeAction {

    /**
     * Handles the specified action properties.
     * @param actionContext The properties of the action to handle.
     */
    fun handle(actionContext: ActionContext)
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
 * Everything an action needs while it runs, handed to it by the tree.
 * @param instanceName instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item associated with the action.
 * @param coroutineScope Coroutine scope for executing asynchronous operations.
 * @property onFindVariable Lambda function for retrieving a [NativeVariableModel] by its key.
 * @param onUpdateVariable Callback invoked when a variable changes.
 * @param trigger Trigger model representing the conditions and outcomes of the action.
 * @param onHandleEvent Runs the triggers filed under one of this action's events.
 * @resolveTemplate Fills in whatever that scope reports, leaving the value alone outside one.
 */
data class ActionContext(
    val instanceName: String,
    val listItemIndex: Int,
    val coroutineScope: CoroutineScope,
    val onFindVariable: (String) -> NativeVariableModel?,
    val onUpdateVariable: (NativeVariableModel?) -> Unit,
    val trigger: NativeActionTriggerModel?,
    val onHandleEvent: (String) -> Unit,
    val resolveTemplate: (String?) -> String? = { it },
)

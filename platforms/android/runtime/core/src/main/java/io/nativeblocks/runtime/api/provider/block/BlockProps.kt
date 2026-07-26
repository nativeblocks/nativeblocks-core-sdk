package io.nativeblocks.runtime.api.provider.block

import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel

const val NONE_INDEX = -1

internal typealias BlockComposable = @Composable (blockProps: BlockProps) -> Unit

/**
 * Represents the properties required for rendering and handling a native block.
 * @param instanceName instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item associated with the block (optional).
 * @param onFindVariable Lambda function for retrieving a [NativeVariableModel] by its key.
 * @param onVariableChange Callback invoked when a variable changes (optional).
 * @param onFindAction Lambda function for retrieving a [NativeActionModel] by its event type.
 * @param onHandleAction Callback invoked to handle an action with the given index, action model, and type (optional).
 * @param block The model representing the block to be rendered (optional).
 * @param onSubBlock Composable callback to render a slot's sub-blocks; receives child block keys grouped by slot name, the slot, index, and scope (optional).
 */
@Immutable
data class BlockProps(
    val instanceName: String,
    val listItemIndex: Int,
    val onFindVariable: (String) -> NativeVariableModel?,
    val onVariableChange: (NativeVariableModel) -> Unit,
    val onFindAction: (String) -> NativeActionModel?,
    val onHandleAction: (Int, NativeActionModel?, String) -> Unit,
    val block: NativeBlockModel,
    val onSubBlock: @Composable (blockKeys: Map<String, List<String>>, slot: NativeBlockSlotModel, index: Int, scope: Any?) -> Unit
)
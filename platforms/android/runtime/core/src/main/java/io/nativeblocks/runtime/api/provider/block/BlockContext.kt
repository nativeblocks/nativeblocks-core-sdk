package io.nativeblocks.runtime.api.provider.block

import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockDataModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel

const val NONE_INDEX = -1

internal typealias BlockComposable = @Composable (blockContext: BlockContext) -> Unit

/**
 * Represents the properties required for rendering and handling a native block.
 * @param instanceName instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item associated with the block (optional).
 * @param onFindVisibility Lambda function resolving the block's visibility value.
 * @param onFindVariable Lambda function resolving the variable a block data entry points at.
 * @param onUpdateVariable Callback invoked to write a value back to the variable a block data entry points at.
 * @param onFindAction Lambda function for retrieving a [NativeActionModel] by its event type.
 * @param onHandleAction Callback invoked to handle an action with the given index, action model, and type (optional).
 * @param block The model representing the block to be rendered (optional).
 * @param onSubBlock Composable callback to render a slot's sub-blocks; receives child block keys grouped by slot name, the slot, index, and scope (optional).
 */
@Immutable
data class BlockContext(
    val instanceName: String,
    val listItemIndex: Int,
    val onFindVisibility: () -> String?,
    val onFindVariable: (NativeBlockDataModel?) -> String?,
    val onUpdateVariable: (NativeBlockDataModel?, String) -> Unit,
    val onFindAction: (String) -> NativeActionModel?,
    val onHandleAction: (Int, NativeActionModel?, String) -> Unit,
    val block: NativeBlockModel,
    val onSubBlock: @Composable (blockKeys: Map<String, List<String>>, slot: NativeBlockSlotModel, index: Int, scope: Any?) -> Unit
)
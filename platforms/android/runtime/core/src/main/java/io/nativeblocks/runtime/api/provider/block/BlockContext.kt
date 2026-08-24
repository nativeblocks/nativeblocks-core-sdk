package io.nativeblocks.runtime.api.provider.block

import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.runtime.Immutable
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockDataModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel

const val NONE_INDEX = -1

internal typealias BlockComposable = @Composable (blockContext: BlockContext) -> Unit

/**
 * Everything a block needs while it renders, handed to it by the tree.
 *
 * @param instanceName Instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item this block belongs to, or [NONE_INDEX] outside a list.
 * @param onFindVisibility Resolves the block's visibility value.
 * @param onFindVariable Resolves the value a block data entry points at.
 * @param onUpdateVariable Writes a value back to the variable a block data entry points at.
 * @param onFindAction Finds the action bound to the given event type.
 * @param onHandleAction Runs an action for the given list item index and event type.
 * @param block The block being rendered.
 * @param modifier Modifiers attached to the block, already ordered and scope checked.
 * @param onSubBlock Renders the child blocks of a slot, handing them the index and the slot's scope.
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
    val modifier: Modifier,
    val onSubBlock: @Composable (blockKeys: Map<String, List<String>>, slot: NativeBlockSlotModel, index: Int, scope: Any?) -> Unit
)
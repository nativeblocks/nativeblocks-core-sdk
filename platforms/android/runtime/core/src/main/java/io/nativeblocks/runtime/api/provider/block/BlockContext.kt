package io.nativeblocks.runtime.api.provider.block

import androidx.compose.runtime.Composable
import androidx.compose.runtime.Immutable
import androidx.compose.ui.Modifier
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockDataModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel

const val NONE_INDEX = -1

sealed interface NativeBlock {

    /** Produces UI where it sits. */
    fun interface Rendering : NativeBlock {
        @Composable
        fun Render(blockContext: BlockContext)
    }

    /** Says what will exist in the slot it sits in, into the scope that slot hands it. */
    fun interface Describing : NativeBlock {
        fun describe(blockContext: BlockContext, scope: Any)
    }
}

/**
 * Everything a block needs while it renders, handed to it by the tree.
 *
 * @param instanceName Instance name of NativeblocksManager.
 * @param listItemIndex Index of the list item this block belongs to, or [NONE_INDEX] outside a list.
 * @param onFindVariable Resolves the value a block data entry points at.
 * @param onUpdateVariable Writes a value back to the variable a block data entry points at.
 * @param onFindAction Finds the action bound to the given event type.
 * @param onHandleAction Runs an action for the given list item index and event type.
 * @param block The block being rendered.
 * @param modifier Modifiers attached to the block, already ordered and scope checked.
 * @param onSubBlock Renders the child blocks of a slot, handing them the index and the slot's scope.
 * @param onDescribeSubBlock Lets the child blocks of a describing slot say what will exist.
 * @param scope The scope this block sits in, or null outside one.
 * @param resolveTemplate Fills in whatever that scope reports, leaving the value alone outside one.
 */
@Immutable
data class BlockContext(
    val instanceName: String,
    val listItemIndex: Int,
    val onFindVariable: (NativeBlockDataModel?) -> String?,
    val onUpdateVariable: (NativeBlockDataModel?, String) -> Unit,
    val onFindAction: (String) -> NativeActionModel?,
    val onHandleAction: (Int, NativeActionModel?, String) -> Unit,
    val block: NativeBlockModel,
    val modifier: Modifier,
    val onSubBlock: @Composable (blockKeys: Map<String, List<String>>, slot: NativeBlockSlotModel, index: Int, scope: Any?) -> Unit,
    val onDescribeSubBlock: (blockKeys: Map<String, List<String>>, slot: NativeBlockSlotModel, scope: Any) -> Unit = { _, _, _ -> },
    val scope: Any? = null,
    val resolveTemplate: (String?) -> String? = { it },
)
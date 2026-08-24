package io.nativeblocks.runtime.api.provider.modifier

import androidx.compose.runtime.Immutable
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModifierDataModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModifierModel

/**
 * Everything a modifier needs while it decorates the block it is attached to.
 *
 * @param instanceName Name of the Nativeblocks instance this modifier belongs to.
 * @param listItemIndex Index of the list item the host block belongs to, or NONE_INDEX outside a list.
 * @param onFindVariable Reads the current value of the variable a modifier data entry points at.
 * @param onUpdateVariable Writes a value back to the variable a modifier data entry points at.
 * @param onFindAction Finds the action bound to the given event type.
 * @param onHandleAction Runs an action for the given list item index and event type.
 * @param modifier The modifier being applied.
 * @param scope Layout scope of the slot the host block sits in, or null when it declares none.
 */
@Immutable
data class ModifierContext(
    val instanceName: String,
    val listItemIndex: Int,
    val onFindVariable: (NativeBlockModifierDataModel?) -> String?,
    val onUpdateVariable: (NativeBlockModifierDataModel?, String) -> Unit,
    val onFindAction: (String) -> NativeActionModel?,
    val onHandleAction: (Int, NativeActionModel?, String) -> Unit,
    val modifier: NativeBlockModifierModel,
    val scope: Any?,
)

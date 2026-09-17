package io.nativeblocks.foundation.block

import androidx.compose.runtime.Composable
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockIndex
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.foundation.util.LazyListDescribeScope
import io.nativeblocks.foundation.util.readListItems
import io.nativeblocks.runtime.api.provider.block.BlockContext

@Block(
    keyType = "nativeblocks/items",
    name = "Items",
    description = "One row per element of a list.",
    scope = "LIST",
    version = 1,
    versionName = "1",
)
internal fun items(
    blockContext: BlockContext,
    describeScope: Any,
    @BlockData(
        description = "The list to repeat over, as JSON.",
        defaultValue = "[]",
    ) list: String = "[]",
    @BlockData(
        description = "Path to a value that tells rows apart, e.g. \"id\". Falls back to position.",
        defaultValue = "",
    ) id: String = "",
    @BlockData(
        description = "Names the element a row is built for; read through the slot, never set.",
        defaultValue = "",
    ) item: String = "",
    @BlockSlot(
        description = "Built once per element; receives the element it belongs to.",
        dataBindings = ["item"],
    ) content: @Composable (index: BlockIndex, scope: Any) -> Unit,
) {
    val listScope = describeScope as? LazyListDescribeScope ?: return
    val rows = listScope.cached(blockContext.block.key, list to id) { readListItems(list, id) }
    listScope.items(
        count = rows.size,
        key = { rows[it].id },
        root = blockContext.block.key,
        element = { rows[it].value },
    ) { index, scope ->
        content(index, scope)
    }
}

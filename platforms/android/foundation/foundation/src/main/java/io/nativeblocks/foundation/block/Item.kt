package io.nativeblocks.foundation.block

import androidx.compose.runtime.Composable
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockIndex
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.foundation.util.LazyListDescribeScope
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX

@Block(
    keyType = "nativeblocks/item",
    name = "Item",
    description = "One item of fixed content inside a list.",
    scope = "LIST",
    version = 1,
    versionName = "1",
)
internal fun item(
    blockContext: BlockContext,
    describeScope: Any,
    @BlockSlot(
        description = "The content of this item.",
    ) content: @Composable (index: BlockIndex, scope: Any) -> Unit,
) {
    val listScope = describeScope as? LazyListDescribeScope ?: return
    listScope.item(blockContext.block.key) { scope -> content(NONE_INDEX, scope) }
}

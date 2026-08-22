package io.nativeblocks.runtime.api.provider.block.defaults

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX

@Composable
internal fun RootBlock(blockContext: BlockContext) {
    val slots = blockContext.block.slots

    val contentSlot = slots["content"]

    Column {
        if (contentSlot != null) {
            blockContext.onSubBlock.invoke(
                blockContext.block.subBlocks.orEmpty(),
                contentSlot,
                NONE_INDEX,
                null,
            )
        }
    }
}
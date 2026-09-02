package io.nativeblocks.foundation.block

import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockIndex
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import androidx.compose.foundation.layout.Box as ComposeBox

@Block(
    keyType = "nativeblocks/box",
    name = "Box",
    description = "Stacking layout container; style it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun Box(
    blockContext: BlockContext,
    @BlockData(
        description = "Children alignment (topStart, topCenter, topEnd, centerStart, center, " +
            "centerEnd, bottomStart, bottomCenter, bottomEnd).",
        defaultValue = "topStart",
    ) contentAlignment: Alignment = Alignment.TopStart,
    @BlockSlot(
        description = "Slot for composing child content within the box.",
    ) content: @Composable (index: BlockIndex, scope: Any) -> Unit,
) {
    ComposeBox(
        modifier = blockContext.modifier,
        contentAlignment = contentAlignment,
    ) {
        content(NONE_INDEX, this)
    }
}

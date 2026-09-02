package io.nativeblocks.foundation.block

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockIndex
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import androidx.compose.foundation.layout.Column as ComposeColumn

@Block(
    keyType = "nativeblocks/column",
    name = "Column",
    description = "Vertical layout container; style it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun Column(
    blockContext: BlockContext,
    @BlockData(
        description = "Vertical arrangement of children (top, bottom, center, spaceBetween, spaceAround, spaceEvenly).",
        defaultValue = "top",
    ) verticalArrangement: Arrangement.Vertical = Arrangement.Top,
    @BlockData(
        description = "Horizontal alignment of children (start, end, centerHorizontally).",
        defaultValue = "start",
    ) horizontalAlignment: Alignment.Horizontal = Alignment.Start,
    @BlockSlot(
        description = "Slot for composing child content within the column.",
    ) content: @Composable (index: BlockIndex, scope: Any) -> Unit,
) {
    ComposeColumn(
        modifier = blockContext.modifier,
        verticalArrangement = verticalArrangement,
        horizontalAlignment = horizontalAlignment,
    ) {
        content(NONE_INDEX, this)
    }
}

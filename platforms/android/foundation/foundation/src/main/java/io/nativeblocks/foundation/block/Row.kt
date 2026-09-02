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
import androidx.compose.foundation.layout.Row as ComposeRow
@Block(
    keyType = "nativeblocks/row",
    name = "Row",
    description = "Horizontal layout container; style it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun Row(
    blockContext: BlockContext,
    @BlockData(
        description = "Horizontal arrangement (start, end, center, spaceBetween, spaceAround, spaceEvenly).",
        defaultValue = "start",
    ) horizontalArrangement: Arrangement.Horizontal = Arrangement.Start,
    @BlockData(
        description = "Vertical alignment of children (top, bottom, centerVertically).",
        defaultValue = "top",
    ) verticalAlignment: Alignment.Vertical = Alignment.Top,
    @BlockSlot(
        description = "Slot for composing child content within the row.",
    ) content: @Composable (index: BlockIndex, scope: Any) -> Unit,
) {
    ComposeRow(
        modifier = blockContext.modifier,
        horizontalArrangement = horizontalArrangement,
        verticalAlignment = verticalAlignment,
    ) {
        content(NONE_INDEX, this)
    }
}

package io.nativeblocks.foundation.block

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockSlot
import io.nativeblocks.foundation.util.LazyListDescribeScope
import io.nativeblocks.runtime.api.provider.block.BlockContext
import androidx.compose.foundation.lazy.LazyRow as ComposeLazyRow

@Block(
    keyType = "nativeblocks/lazy_row",
    name = "Lazy Row",
    description = "Scrollable horizontal list; style it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun LazyRow(
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
        description = "Slot describing what the list contains; a block inside it says what will exist.",
        scope = "LIST",
    ) content: (describeScope: Any) -> Unit,
) {
    // The memory a describing block borrows: it belongs to this container, so what a
    // block keeps in it lives exactly as long as the container does.
    val describeMemory = remember { mutableMapOf<Any, Pair<Any?, Any?>>() }

    ComposeLazyRow(
        modifier = blockContext.modifier,
        state = rememberLazyListState(),
        horizontalArrangement = horizontalArrangement,
        verticalAlignment = verticalAlignment,
    ) {
        content(LazyListDescribeScope(this, describeMemory, blockContext.scope))
    }
}

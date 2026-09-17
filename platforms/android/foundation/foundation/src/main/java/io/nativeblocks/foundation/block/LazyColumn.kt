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
import androidx.compose.foundation.lazy.LazyColumn as ComposeLazyColumn

@Block(
    keyType = "nativeblocks/lazy_column",
    name = "Lazy Column",
    description = "Scrollable vertical list; style it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun LazyColumn(
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
        description = "Slot describing what the list contains; a block inside it says what will exist.",
        scope = "LIST",
    ) content: (describeScope: Any) -> Unit,
) {
    val describeMemory = remember { mutableMapOf<Any, Pair<Any?, Any?>>() }

    ComposeLazyColumn(
        modifier = blockContext.modifier,
        state = rememberLazyListState(),
        verticalArrangement = verticalArrangement,
        horizontalAlignment = horizontalAlignment,
    ) {
        content(LazyListDescribeScope(this, describeMemory, blockContext.scope))
    }
}

package io.nativeblocks.foundation.block

import androidx.compose.runtime.Composable
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.runtime.api.provider.block.BlockContext
import androidx.compose.foundation.layout.Spacer as ComposeSpacer

@Block(
    keyType = "nativeblocks/spacer",
    name = "Spacer",
    description = "Empty space; size it by attaching width/height/weight modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun Spacer(blockContext: BlockContext) {
    ComposeSpacer(modifier = blockContext.modifier)
}

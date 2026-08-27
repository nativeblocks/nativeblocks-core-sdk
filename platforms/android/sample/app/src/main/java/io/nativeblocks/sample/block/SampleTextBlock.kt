package io.nativeblocks.sample.block

import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.sp
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.compiler.type.BlockProp
import io.nativeblocks.runtime.api.provider.block.BlockContext

@Block(
    keyType = "SAMPLE_TEXT",
    name = "Sample text",
    description = "Renders a line of text supplied by the frame",
)
@Composable
fun SampleText(
    blockContext: BlockContext? = null,
    @BlockData(description = "Text to render")
    text: String,
    @BlockData(description = "Font size in sp", defaultValue = "16")
    fontSize: Int,
    @BlockProp(description = "Render the text in bold", defaultValue = "false")
    bold: Boolean,
    @BlockData(description = "Render the text in bold", defaultValue = "12")
    someValueForDp: Dp,
) {
    Text(
        text = text,
        fontSize = fontSize.sp,
        fontWeight = if (bold) FontWeight.Bold else FontWeight.Normal,
        modifier = blockContext?.modifier ?: Modifier
    )
}

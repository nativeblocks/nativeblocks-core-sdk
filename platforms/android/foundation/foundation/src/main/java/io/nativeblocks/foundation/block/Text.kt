package io.nativeblocks.foundation.block

import androidx.compose.foundation.text.BasicText
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.sp
import io.nativeblocks.compiler.type.Block
import io.nativeblocks.compiler.type.BlockData
import io.nativeblocks.runtime.api.provider.block.BlockContext

@Block(
    keyType = "nativeblocks/text",
    name = "Text",
    description = "Text content; style the space around it by attaching modifiers.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun Text(
    blockContext: BlockContext,
    @BlockData(
        description = "The text content to display.",
    ) text: String,
    @BlockData(
        description = "Font size in SP.",
        defaultValue = "14.0",
    ) fontSize: TextUnit = 14.sp,
    @BlockData(
        description = "Text color in hexadecimal format.",
        defaultValue = "#ff000000",
    ) color: Color = Color.Black,
    @BlockData(
        description = "Font weight (thin, extraLight, light, normal, medium, semiBold, bold, extraBold, black).",
        defaultValue = "normal",
    ) fontWeight: FontWeight = FontWeight.Normal,
    @BlockData(
        description = "Text alignment (start, center, end, justify).",
        defaultValue = "start",
    ) textAlign: TextAlign = TextAlign.Start,
    @BlockData(
        description = "Overflow behaviour when the text does not fit (clip, ellipsis, visible).",
        defaultValue = "clip",
    ) overflow: TextOverflow = TextOverflow.Clip,
    @BlockData(
        description = "Minimum number of lines to display.",
        defaultValue = "1",
    ) minLines: Int = 1,
    @BlockData(
        description = "Maximum number of lines to display.",
        defaultValue = "9999",
    ) maxLines: Int = 9999,
) {
    BasicText(
        text = text,
        modifier = blockContext.modifier,
        style = TextStyle(
            color = color,
            fontSize = fontSize,
            fontWeight = fontWeight,
            textAlign = textAlign,
        ),
        overflow = overflow,
        minLines = minLines,
        maxLines = maxLines,
    )
}

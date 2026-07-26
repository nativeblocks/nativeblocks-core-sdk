package io.nativeblocks.runtime.api.provider.block.defaults

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.core.graphics.toColorInt
import io.nativeblocks.runtime.api.provider.block.BlockProps
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import io.nativeblocks.runtime.api.util.LocalNativeWindowWidthClass
import io.nativeblocks.runtime.api.util.findWindowSizeClass

@Composable
internal fun RootBlock(blockProps: BlockProps) {
    val properties = blockProps.block.properties
    val slots = blockProps.block.slots

    val windowManager = LocalNativeWindowWidthClass.current
    val paddingStart = findWindowSizeClass(properties["paddingStart"], windowManager)
    val paddingTop = findWindowSizeClass(properties["paddingTop"], windowManager)
    val paddingEnd = findWindowSizeClass(properties["paddingEnd"], windowManager)
    val paddingBottom = findWindowSizeClass(properties["paddingBottom"], windowManager)
    val backgroundColor = findWindowSizeClass(properties["backgroundColor"], windowManager) ?: "#ffffffff"

    val contentSlot = slots["content"]

    val modifier = Modifier
        .background(Color(backgroundColor.toColorInt()))
        .padding(spacingMapper(listOf(paddingStart, paddingTop, paddingEnd, paddingBottom)))

    Column(
        modifier = modifier,
        verticalArrangement = findArrangementVertical(
            findWindowSizeClass(
                properties["verticalArrangement"],
                windowManager
            )
        ),
        horizontalAlignment = findAlignmentHorizontal(
            findWindowSizeClass(
                properties["horizontalAlignment"],
                windowManager
            )
        )
    ) {
        if (contentSlot != null) {
            blockProps.onSubBlock.invoke(
                blockProps.block.subBlocks.orEmpty(),
                contentSlot,
                NONE_INDEX,
                this
            )
        }
    }
}

private fun findArrangementVertical(arrangement: String?): Arrangement.Vertical {
    return if (arrangement?.toFloatOrNull() != null) {
        Arrangement.spacedBy(arrangement.toFloatOrNull()?.dp ?: 0.dp)
    } else {
        when (arrangement) {
            "top" -> Arrangement.Top
            "bottom" -> Arrangement.Bottom
            "center" -> Arrangement.Center
            "spaceBetween" -> Arrangement.SpaceBetween
            "spaceAround" -> Arrangement.SpaceAround
            "spaceEvenly" -> Arrangement.SpaceEvenly
            else -> Arrangement.Top
        }
    }
}

private fun findAlignmentHorizontal(alignment: String?): Alignment.Horizontal {
    return when (alignment) {
        "start" -> Alignment.Start
        "end" -> Alignment.End
        "centerHorizontally" -> Alignment.CenterHorizontally
        else -> Alignment.Start
    }
}

private fun spacingMapper(paddings: List<String?>): PaddingValues {
    return PaddingValues(
        start = paddings[0]?.toFloatOrNull()?.dp ?: 0.dp,
        top = paddings[1]?.toFloatOrNull()?.dp ?: 0.dp,
        end = paddings[2]?.toFloatOrNull()?.dp ?: 0.dp,
        bottom = paddings[3]?.toFloatOrNull()?.dp ?: 0.dp,
    )
}
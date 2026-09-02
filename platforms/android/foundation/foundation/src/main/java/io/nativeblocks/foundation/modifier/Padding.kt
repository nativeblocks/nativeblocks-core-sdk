package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.padding
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/padding",
    name = "Padding",
    description = "Adds padding around the block.",
    version = 1,
    versionName = "1",
)
internal fun padding(
    @ModifierData(description = "Padding on the start side in DP.", defaultValue = "0.0")
    start: Dp = 0.dp,
    @ModifierData(description = "Padding on the top side in DP.", defaultValue = "0.0")
    top: Dp = 0.dp,
    @ModifierData(description = "Padding on the end side in DP.", defaultValue = "0.0")
    end: Dp = 0.dp,
    @ModifierData(description = "Padding on the bottom side in DP.", defaultValue = "0.0")
    bottom: Dp = 0.dp,
): UiModifier = UiModifier.padding(start = start, top = top, end = end, bottom = bottom)

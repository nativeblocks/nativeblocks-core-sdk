package io.nativeblocks.sample.modifier

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.padding
import androidx.compose.ui.Modifier as ComposeModifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@Modifier(
    keyType = "nativeblocks/padding",
    name = "Padding",
    description = "Applies padding on each side of the block.",
)
fun NativePadding(
    modifierContext: ModifierContext? = null,
    @ModifierData(
        description = "Padding on the start side in DP.",
        defaultValue = "0.0"
    ) paddingStart: Dp = 0.dp,
    @ModifierData(
        description = "Padding on the top side in DP.",
        defaultValue = "0.0"
    ) paddingTop: Dp = 0.dp,
    @ModifierData(
        description = "Padding on the end side in DP.",
        defaultValue = "0.0"
    ) paddingEnd: Dp = 0.dp,
    @ModifierData(
        description = "Padding on the bottom side in DP.",
        defaultValue = "0.0"
    ) paddingBottom: Dp = 0.dp,
): ComposeModifier {
    return ComposeModifier.padding(
        start = paddingStart,
        top = paddingTop,
        end = paddingEnd,
        bottom = paddingBottom,
    ).background(Color.Gray.copy(alpha = 0.3f))
}

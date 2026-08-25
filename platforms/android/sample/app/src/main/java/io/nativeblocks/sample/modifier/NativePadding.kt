package io.nativeblocks.sample.modifier

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.padding
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.NativeModifier
import io.nativeblocks.compiler.type.NativeModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@NativeModifier(
    keyType = "nativeblocks/padding",
    name = "Padding",
    description = "Applies padding on each side of the block.",
)
fun NativePadding(
    modifierContext: ModifierContext? = null,
    @NativeModifierData(
        description = "Padding on the start side in DP.",
        defaultValue = "0.0"
    ) paddingStart: Dp = 0.dp,
    @NativeModifierData(
        description = "Padding on the top side in DP.",
        defaultValue = "0.0"
    ) paddingTop: Dp = 0.dp,
    @NativeModifierData(
        description = "Padding on the end side in DP.",
        defaultValue = "0.0"
    ) paddingEnd: Dp = 0.dp,
    @NativeModifierData(
        description = "Padding on the bottom side in DP.",
        defaultValue = "0.0"
    ) paddingBottom: Dp = 0.dp,
): Modifier {
    return Modifier.padding(
        start = paddingStart,
        top = paddingTop,
        end = paddingEnd,
        bottom = paddingBottom,
    ).background(Color.Gray.copy(alpha = 0.3f))
}

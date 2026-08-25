package io.nativeblocks.sample.modifier

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.NativeModifier
import io.nativeblocks.compiler.type.NativeModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@NativeModifier(
    keyType = "SAMPLE_SHADOW",
    name = "Sample shadow",
    description = "Drops a rounded shadow behind the block it is attached to",
)
fun SampleShadow(
    modifierContext: ModifierContext? = null,
    @NativeModifierData(description = "Shadow elevation in DP", defaultValue = "4")
    elevation: Dp = 4.dp,
    @NativeModifierData(description = "Corner radius in DP", defaultValue = "8")
    cornerRadius: Dp = 8.dp,
): Modifier {
    return Modifier.shadow(
        elevation = elevation,
        shape = RoundedCornerShape(cornerRadius),
    )
}

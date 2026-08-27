package io.nativeblocks.sample.modifier

import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.Modifier as ComposeModifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@Modifier(
    keyType = "SAMPLE_SHADOW",
    name = "Sample shadow",
    description = "Drops a rounded shadow behind the block it is attached to",
)
fun SampleShadow(
    modifierContext: ModifierContext? = null,
    @ModifierData(description = "Shadow elevation in DP", defaultValue = "4")
    elevation: Dp = 4.dp,
    @ModifierData(description = "Corner radius in DP", defaultValue = "8")
    cornerRadius: Dp = 8.dp,
): ComposeModifier {
    return ComposeModifier.shadow(
        elevation = elevation,
        shape = RoundedCornerShape(cornerRadius),
    )
}

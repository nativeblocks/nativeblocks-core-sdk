package io.nativeblocks.sample.modifier

import androidx.compose.foundation.clickable
import androidx.compose.ui.Modifier as ComposeModifier
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.compiler.type.ModifierEvent
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@Modifier(
    keyType = "SAMPLE_TAP",
    name = "Sample tap",
    description = "Makes the block it is attached to tappable",
)
fun SampleTap(
    modifierContext: ModifierContext? = null,
    @ModifierData(description = "Whether the tap is enabled", defaultValue = "true")
    enabled: Boolean = true,
    @ModifierEvent(description = "Triggered when the block is tapped")
    onTap: (() -> Unit)? = null,
): ComposeModifier {
    return if (onTap != null) {
        ComposeModifier.clickable(enabled = enabled) { onTap() }
    } else {
        ComposeModifier
    }
}

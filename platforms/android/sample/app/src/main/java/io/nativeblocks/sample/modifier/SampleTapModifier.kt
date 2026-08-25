package io.nativeblocks.sample.modifier

import androidx.compose.foundation.clickable
import androidx.compose.ui.Modifier
import io.nativeblocks.compiler.type.NativeModifier
import io.nativeblocks.compiler.type.NativeModifierData
import io.nativeblocks.compiler.type.NativeModifierEvent
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@NativeModifier(
    keyType = "SAMPLE_TAP",
    name = "Sample tap",
    description = "Makes the block it is attached to tappable",
)
fun SampleTap(
    modifierContext: ModifierContext? = null,
    @NativeModifierData(description = "Whether the tap is enabled", defaultValue = "true")
    enabled: Boolean = true,
    @NativeModifierEvent(description = "Triggered when the block is tapped")
    onTap: (() -> Unit)? = null,
): Modifier {
    return if (onTap != null) {
        Modifier.clickable(enabled = enabled) { onTap() }
    } else {
        Modifier
    }
}

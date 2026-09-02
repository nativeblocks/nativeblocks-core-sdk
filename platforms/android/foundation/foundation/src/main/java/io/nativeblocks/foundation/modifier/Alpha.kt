package io.nativeblocks.foundation.modifier

import androidx.compose.ui.draw.alpha
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/alpha",
    name = "Alpha",
    description = "Sets the opacity of the block.",
    version = 1,
    versionName = "1",
)
internal fun alpha(
    @ModifierData(description = "Opacity between 0.0 (transparent) and 1.0 (opaque).", defaultValue = "1.0")
    value: Float = 1f,
): UiModifier = UiModifier.alpha(value.coerceIn(0f, 1f))

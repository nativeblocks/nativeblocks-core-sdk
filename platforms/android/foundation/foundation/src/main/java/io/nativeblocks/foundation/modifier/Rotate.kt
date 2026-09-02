package io.nativeblocks.foundation.modifier

import androidx.compose.ui.draw.rotate
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/rotate",
    name = "Rotate",
    description = "Rotates the block around its center.",
    version = 1,
    versionName = "1",
)
internal fun rotate(
    @ModifierData(description = "Rotation in degrees, clockwise.", defaultValue = "0.0")
    degrees: Float = 0f,
): UiModifier = UiModifier.rotate(degrees)

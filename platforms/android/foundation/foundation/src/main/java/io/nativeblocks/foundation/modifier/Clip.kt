package io.nativeblocks.foundation.modifier

import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.foundation.util.SHAPE_RECTANGLE
import io.nativeblocks.foundation.util.cornerShape
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/clip",
    name = "Clip",
    description = "Clips the block content to a shape.",
    version = 1,
    versionName = "1",
)
internal fun clip(
    @ModifierData(description = "Shape to clip to (rectangle, circle).", defaultValue = "rectangle")
    shape: String = SHAPE_RECTANGLE,
    @ModifierData(description = "Top-start corner radius in DP.", defaultValue = "0.0")
    radiusTopStart: Dp = 0.dp,
    @ModifierData(description = "Top-end corner radius in DP.", defaultValue = "0.0")
    radiusTopEnd: Dp = 0.dp,
    @ModifierData(description = "Bottom-start corner radius in DP.", defaultValue = "0.0")
    radiusBottomStart: Dp = 0.dp,
    @ModifierData(description = "Bottom-end corner radius in DP.", defaultValue = "0.0")
    radiusBottomEnd: Dp = 0.dp,
): UiModifier = UiModifier.clip(
    cornerShape(shape, radiusTopStart, radiusTopEnd, radiusBottomStart, radiusBottomEnd),
)

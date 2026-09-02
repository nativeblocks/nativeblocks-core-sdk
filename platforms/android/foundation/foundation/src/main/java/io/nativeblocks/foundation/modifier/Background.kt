package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.background
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.foundation.util.SHAPE_RECTANGLE
import io.nativeblocks.foundation.util.cornerShape
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/background",
    name = "Background",
    description = "Fills the block background with a color.",
    version = 1,
    versionName = "1",
)
internal fun background(
    @ModifierData(
        description = "Background color in hexadecimal format.",
        defaultValue = "#00000000",
    ) color: Color = Color.Transparent,
    @ModifierData(description = "Shape of the background (rectangle, circle).", defaultValue = "rectangle")
    shape: String = SHAPE_RECTANGLE,
    @ModifierData(description = "Top-start corner radius in DP.", defaultValue = "0.0")
    radiusTopStart: Dp = 0.dp,
    @ModifierData(description = "Top-end corner radius in DP.", defaultValue = "0.0")
    radiusTopEnd: Dp = 0.dp,
    @ModifierData(description = "Bottom-start corner radius in DP.", defaultValue = "0.0")
    radiusBottomStart: Dp = 0.dp,
    @ModifierData(description = "Bottom-end corner radius in DP.", defaultValue = "0.0")
    radiusBottomEnd: Dp = 0.dp,
): UiModifier = UiModifier.background(
    color = color,
    shape = cornerShape(shape, radiusTopStart, radiusTopEnd, radiusBottomStart, radiusBottomEnd),
)

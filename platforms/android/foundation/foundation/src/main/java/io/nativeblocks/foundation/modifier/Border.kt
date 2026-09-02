package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.border
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.foundation.util.SHAPE_RECTANGLE
import io.nativeblocks.foundation.util.cornerShape
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/border",
    name = "Border",
    description = "Draws a border around the block.",
    version = 1,
    versionName = "1",
)
internal fun border(
    @ModifierData(description = "Border width in DP.", defaultValue = "1.0")
    width: Dp = 1.dp,
    @ModifierData(
        description = "Border color in hexadecimal format.",
        defaultValue = "#00000000",
    ) color: Color = Color.Transparent,
    @ModifierData(description = "Shape of the border (rectangle, circle).", defaultValue = "rectangle")
    shape: String = SHAPE_RECTANGLE,
    @ModifierData(description = "Top-start corner radius in DP.", defaultValue = "0.0")
    radiusTopStart: Dp = 0.dp,
    @ModifierData(description = "Top-end corner radius in DP.", defaultValue = "0.0")
    radiusTopEnd: Dp = 0.dp,
    @ModifierData(description = "Bottom-start corner radius in DP.", defaultValue = "0.0")
    radiusBottomStart: Dp = 0.dp,
    @ModifierData(description = "Bottom-end corner radius in DP.", defaultValue = "0.0")
    radiusBottomEnd: Dp = 0.dp,
): UiModifier = UiModifier.border(
    width = width,
    color = color,
    shape = cornerShape(shape, radiusTopStart, radiusTopEnd, radiusBottomStart, radiusBottomEnd),
)

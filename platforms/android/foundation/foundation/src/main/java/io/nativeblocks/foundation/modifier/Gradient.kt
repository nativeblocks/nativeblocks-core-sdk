package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.background
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.foundation.util.GRADIENT_LINEAR
import io.nativeblocks.foundation.util.SHAPE_RECTANGLE
import io.nativeblocks.foundation.util.cornerShape
import io.nativeblocks.foundation.util.gradientBrush
import io.nativeblocks.foundation.util.parseColors
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/gradient",
    name = "Gradient",
    description = "Fills the block background with a gradient of colors.",
    version = 1,
    versionName = "1",
)
internal fun gradient(
    @ModifierData(
        description = "Comma-separated hexadecimal colors (e.g. '#004FF0, #00FFFFFF').",
        defaultValue = "",
    ) colors: String = "",
    @ModifierData(description = "Gradient type (linear, radial, sweep).", defaultValue = "linear")
    type: String = GRADIENT_LINEAR,
    @ModifierData(
        description = "Angle in degrees for linear gradients; 0 flows start-to-end, 90 top-to-bottom.",
        defaultValue = "0.0",
    ) angle: Float = 0f,
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
): UiModifier {
    val colorList = parseColors(colors)
    val backgroundShape = cornerShape(shape, radiusTopStart, radiusTopEnd, radiusBottomStart, radiusBottomEnd)
    return when {
        colorList.isEmpty() -> UiModifier
        colorList.size == 1 -> UiModifier.background(colorList.first(), backgroundShape)
        else -> UiModifier.background(gradientBrush(type, colorList, angle), backgroundShape)
    }
}

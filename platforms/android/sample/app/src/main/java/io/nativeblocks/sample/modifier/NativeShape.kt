package io.nativeblocks.sample.modifier

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.Modifier as ComposeModifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.sample.modifier.ShapeStyle
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@Modifier(
    keyType = "nativeblocks/shape",
    name = "Shape",
    description = "Clips the block to a shape and paints its background, border and shadow.",
)
fun NativeShape(
    modifierContext: ModifierContext? = null,
    @ModifierData(
        description = "Outline of the shape: rectangle, roundedRectangle, circle or capsule.",
        defaultValue = "rectangle"
    ) style: ShapeStyle = ShapeStyle.Rectangle,
    @ModifierData(description = "Top-start corner radius in DP.", defaultValue = "0")
    radiusTopStart: Dp = 0.dp,
    @ModifierData(description = "Top-end corner radius in DP.", defaultValue = "0")
    radiusTopEnd: Dp = 0.dp,
    @ModifierData(description = "Bottom-start corner radius in DP.", defaultValue = "0")
    radiusBottomStart: Dp = 0.dp,
    @ModifierData(description = "Bottom-end corner radius in DP.", defaultValue = "0")
    radiusBottomEnd: Dp = 0.dp,
    @ModifierData(description = "Background color in hex.", defaultValue = "#00000000")
    backgroundColor: Color = Color.Transparent,
    @ModifierData(description = "Border color in hex.", defaultValue = "#00000000")
    borderColor: Color = Color.Transparent,
    @ModifierData(description = "Border width in DP.", defaultValue = "0")
    borderWidth: Dp = 0.dp,
    @ModifierData(description = "Shadow elevation in DP.", defaultValue = "0")
    elevation: Dp = 0.dp,
    @ModifierData(description = "Clip the block's own content to the shape.", defaultValue = "true")
    clipContent: Boolean = true,
): ComposeModifier {
    val shape = shapeOf(
        style = style,
        radiusTopStart = radiusTopStart,
        radiusTopEnd = radiusTopEnd,
        radiusBottomStart = radiusBottomStart,
        radiusBottomEnd = radiusBottomEnd,
    )
    var modifier = ComposeModifier
        .shadow(elevation = elevation, shape = shape)
        .background(color = backgroundColor, shape = shape)
        .border(width = borderWidth, color = borderColor, shape = shape)
    if (clipContent) {
        modifier = modifier.clip(shape)
    }
    return modifier
}

private fun shapeOf(
    style: ShapeStyle,
    radiusTopStart: Dp,
    radiusTopEnd: Dp,
    radiusBottomStart: Dp,
    radiusBottomEnd: Dp,
): Shape {
    return when (style) {
        ShapeStyle.Circle -> CircleShape
        ShapeStyle.Capsule -> RoundedCornerShape(percent = 50)
        ShapeStyle.RoundedRectangle -> RoundedCornerShape(
            topStart = radiusTopStart,
            topEnd = radiusTopEnd,
            bottomStart = radiusBottomStart,
            bottomEnd = radiusBottomEnd,
        )
        ShapeStyle.Rectangle -> RectangleShape
    }
}

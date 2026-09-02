package io.nativeblocks.foundation.util

import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

internal const val SHAPE_RECTANGLE = "rectangle"
internal const val SHAPE_CIRCLE = "circle"

/**
 * Builds a [Shape] from a shape keyword and per-corner radii.
 * Radii only apply to [SHAPE_RECTANGLE]; all-zero radii resolve to a plain rectangle.
 */
internal fun cornerShape(
    shape: String = SHAPE_RECTANGLE,
    topStart: Dp = 0.dp,
    topEnd: Dp = 0.dp,
    bottomStart: Dp = 0.dp,
    bottomEnd: Dp = 0.dp,
): Shape = when (shape) {
    SHAPE_CIRCLE -> CircleShape
    else -> {
        if (listOf(topStart, topEnd, bottomStart, bottomEnd).all { it == 0.dp }) {
            RectangleShape
        } else {
            RoundedCornerShape(
                topStart = topStart,
                topEnd = topEnd,
                bottomStart = bottomStart,
                bottomEnd = bottomEnd,
            )
        }
    }
}

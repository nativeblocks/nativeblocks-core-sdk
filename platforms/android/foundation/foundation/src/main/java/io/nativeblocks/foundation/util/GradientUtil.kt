package io.nativeblocks.foundation.util

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.LinearGradientShader
import androidx.compose.ui.graphics.Shader
import androidx.compose.ui.graphics.ShaderBrush
import androidx.core.graphics.toColorInt
import kotlin.math.abs
import kotlin.math.cos
import kotlin.math.sin

internal const val GRADIENT_LINEAR = "linear"
internal const val GRADIENT_RADIAL = "radial"
internal const val GRADIENT_SWEEP = "sweep"

/**
 * Parses a comma-separated list of hexadecimal colors (e.g. "#004FF0, #00FFFFFF"),
 * dropping entries that fail to parse.
 */
internal fun parseColors(colors: String): List<Color> {
    return colors.split(",").mapNotNull { value ->
        runCatching { Color(value.trim().toColorInt()) }.getOrNull()
    }
}

/**
 * Builds a gradient [Brush] of the given type. Linear gradients honor [angleDegrees],
 * where 0 flows start-to-end and 90 flows top-to-bottom.
 */
internal fun gradientBrush(type: String, colors: List<Color>, angleDegrees: Float): Brush {
    return when (type) {
        GRADIENT_RADIAL -> Brush.radialGradient(colors)
        GRADIENT_SWEEP -> Brush.sweepGradient(colors)
        else -> AngularLinearGradient(colors, angleDegrees)
    }
}

/**
 * A linear gradient that flows at an arbitrary angle across the drawing area,
 * since [Brush.linearGradient] only supports fixed start/end offsets.
 */
internal data class AngularLinearGradient(
    private val colors: List<Color>,
    private val angleDegrees: Float,
) : ShaderBrush() {

    override fun createShader(size: Size): Shader {
        val angleRadians = Math.toRadians(angleDegrees.toDouble())
        val direction = Offset(cos(angleRadians).toFloat(), sin(angleRadians).toFloat())
        // half-length of the rect's projection onto the gradient axis
        val halfLength = (abs(direction.x) * size.width + abs(direction.y) * size.height) / 2f
        val center = Offset(size.width / 2f, size.height / 2f)
        return LinearGradientShader(
            from = center - Offset(direction.x * halfLength, direction.y * halfLength),
            to = center + Offset(direction.x * halfLength, direction.y * halfLength),
            colors = colors,
        )
    }
}

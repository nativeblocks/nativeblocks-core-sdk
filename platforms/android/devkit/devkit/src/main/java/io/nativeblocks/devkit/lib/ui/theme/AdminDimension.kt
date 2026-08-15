package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

internal data class AdminDimension(
    val space0: Dp = 0.dp,
    val space1: Dp = 4.dp,
    val space2: Dp = 8.dp,
    val space3: Dp = 12.dp,
    val space4: Dp = 16.dp,
    val space5: Dp = 20.dp,
    val space6: Dp = 24.dp,
    val space7: Dp = 28.dp,
    val space8: Dp = 32.dp,
    val space9: Dp = 36.dp,
    val space10: Dp = 40.dp,
    val space11: Dp = 44.dp,
    val space12: Dp = 48.dp,
    val space13: Dp = 52.dp,
    val space14: Dp = 56.dp,
    val space15: Dp = 60.dp,
    val space16: Dp = 64.dp,
    val space17: Dp = 68.dp,
    val space18: Dp = 72.dp,
    val space19: Dp = 76.dp,
    val space20: Dp = 80.dp,
    val space21: Dp = 84.dp,
    val space22: Dp = 88.dp,
    val space23: Dp = 92.dp,
    val space24: Dp = 96.dp,
    val space25: Dp = 100.dp,
)

internal val LocalDimensions = staticCompositionLocalOf { AdminDimension() }
package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

internal data class AdminRadius(
    val radius0: Dp = 0.dp,
    val radius1: Dp = 2.dp,
    val radius2: Dp = 4.dp,
    val radius3: Dp = 6.dp,
    val radius4: Dp = 8.dp,
    val radius5: Dp = 10.dp,
    val radius6: Dp = 12.dp,
    val radius7: Dp = 14.dp,
    val radius8: Dp = 16.dp,
    val radius9: Dp = 18.dp,
    val radius10: Dp = 20.dp,
)

internal val LocalRadius = staticCompositionLocalOf { AdminRadius() }
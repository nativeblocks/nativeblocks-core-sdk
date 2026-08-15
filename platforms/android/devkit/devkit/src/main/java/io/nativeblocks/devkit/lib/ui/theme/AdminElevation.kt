package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp

internal data class AdminElevation(
    val elevation0: Dp = 0.dp,
    val elevation1: Dp = 1.dp,
    val elevation2: Dp = 2.dp,
    val elevation3: Dp = 3.dp,
    val elevation4: Dp = 4.dp,
)

internal val LocalElevation = staticCompositionLocalOf { AdminElevation() }
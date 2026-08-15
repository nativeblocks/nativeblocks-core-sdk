package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.ReadOnlyComposable
import androidx.compose.runtime.remember

internal object AdminTheme {

    val colors: AdminColor
        @Composable
        @ReadOnlyComposable
        get() = LocalColors.current

    val typography: AdminTypography
        @Composable
        @ReadOnlyComposable
        get() = LocalTypography.current

    val dimensions: AdminDimension
        @Composable
        @ReadOnlyComposable
        get() = LocalDimensions.current

    val radius: AdminRadius
        @Composable
        @ReadOnlyComposable
        get() = LocalRadius.current

    val elevation: AdminElevation
        @Composable
        @ReadOnlyComposable
        get() = LocalElevation.current

    val icons: AdminIcon
        @Composable
        @ReadOnlyComposable
        get() = LocaleIcon.current
}

@Composable
internal fun AdminTheme(
    colors: AdminColor = AdminTheme.colors,
    typography: AdminTypography = AdminTheme.typography,
    dimensions: AdminDimension = AdminTheme.dimensions,
    elevation: AdminElevation = AdminTheme.elevation,
    radius: AdminRadius = AdminTheme.radius,
    icons: AdminIcon = AdminTheme.icons,
    content: @Composable () -> Unit,
) {
    val rememberedColors = remember { colors.copy() }.apply { updateColors(colors) }
    CompositionLocalProvider(
        LocalColors provides rememberedColors,
        LocalDimensions provides dimensions,
        LocalTypography provides typography,
        LocalElevation provides elevation,
        LocalRadius provides radius,
        LocaleIcon provides icons,
    ) {
        content()
    }
}
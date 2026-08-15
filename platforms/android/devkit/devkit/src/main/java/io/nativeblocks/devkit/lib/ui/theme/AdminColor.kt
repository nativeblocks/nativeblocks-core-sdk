package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.graphics.Color

internal fun lightTheme(): AdminColor {
    return AdminColor(
        primary = Color(0xFF346EE7),
        onPrimary = Color(0xFFFFFFFF),
        error = Color(0xFFC83E2E),
        onError = Color(0xFFFFFFFF),
        success = Color(0xFF079455),
        onSuccess = Color(0xFFFFFFFF),
        warning = Color(0xFFDC6803),
        onWarning = Color(0xFFFFFFFF),
        background = Color(0xFFFFFFFF),
        surface = Color(0xFFF9FAFB),
        foregroundEmphasize = Color(0xFF101828),
        foregroundRegular = Color(0XFF475467),
        outline = Color(0xFFD0D5DD),
        isLight = true
    )
}

internal class AdminColor(
    primary: Color,
    onPrimary: Color,
    error: Color,
    onError: Color,
    success: Color,
    onSuccess: Color,
    warning: Color,
    onWarning: Color,
    background: Color,
    surface: Color,
    foregroundEmphasize: Color,
    foregroundRegular: Color,
    outline: Color,
    isLight: Boolean,
) {
    var primary by mutableStateOf(primary)
        private set

    var onPrimary by mutableStateOf(onPrimary)
        private set

    var error by mutableStateOf(error)
        private set

    var onError by mutableStateOf(onError)
        private set

    var success by mutableStateOf(success)
        private set

    var onSuccess by mutableStateOf(onSuccess)
        private set

    var warning by mutableStateOf(warning)
        private set

    var onWarning by mutableStateOf(onWarning)
        private set

    var background by mutableStateOf(background)
        private set

    var surface by mutableStateOf(surface)
        private set

    var foregroundEmphasize by mutableStateOf(foregroundEmphasize)
        private set

    var foregroundRegular by mutableStateOf(foregroundRegular)
        private set

    var outline by mutableStateOf(outline)
        private set

    var isLight by mutableStateOf(isLight)
        internal set

    fun copy(
        primary: Color = this.primary,
        onPrimary: Color = this.onPrimary,
        error: Color = this.error,
        onError: Color = this.onError,
        success: Color = this.success,
        onSuccess: Color = this.onSuccess,
        warning: Color = this.warning,
        onWarning: Color = this.onWarning,
        background: Color = this.background,
        surface: Color = this.surface,
        foregroundEmphasize: Color = this.foregroundEmphasize,
        foregroundRegular: Color = this.foregroundRegular,
        outline: Color = this.outline,
        isLight: Boolean = this.isLight,
    ): AdminColor = AdminColor(
        primary,
        onPrimary,
        error,
        onError,
        success,
        onSuccess,
        warning,
        onWarning,
        background,
        surface,
        foregroundEmphasize,
        foregroundRegular,
        outline,
        isLight,
    )

    fun updateColors(other: AdminColor) {
        primary = other.primary
        onPrimary = other.onPrimary
        error = other.error
        onError = other.onError
        success = other.success
        onSuccess = other.onSuccess
        warning = other.warning
        onWarning = other.onWarning
        background = other.background
        surface = other.surface
        foregroundEmphasize = other.foregroundEmphasize
        foregroundRegular = other.foregroundRegular
        outline = other.outline
        isLight = other.isLight
    }

}


internal val LocalColors = staticCompositionLocalOf { lightTheme() }

package io.nativeblocks.devkit.lib.ui.theme

import androidx.compose.runtime.staticCompositionLocalOf
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp
import io.nativeblocks.devkit.R

internal val fonts = FontFamily(
    Font(R.font.inter_regular),
    Font(R.font.inter_medium),
    Font(R.font.inter_semibold),
    Font(R.font.inter_bold)
)

internal data class AdminTypography(
    val h1Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 36.sp
    ),
    val h1Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 36.sp
    ),
    val h1SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 36.sp
    ),
    val h1Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 36.sp
    ),
    val h2Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 30.sp
    ),
    val h2Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 30.sp
    ),
    val h2SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 30.sp
    ),
    val h2Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 30.sp
    ),
    val h3Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 24.sp
    ),
    val h3Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 24.sp
    ),
    val h3SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 24.sp
    ),
    val h3Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 24.sp
    ),
    val s1Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 20.sp
    ),
    val s1Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 20.sp
    ),
    val s1SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 20.sp
    ),
    val s1Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 20.sp
    ),
    val s2Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 18.sp
    ),
    val s2Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 18.sp
    ),
    val s2SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 18.sp
    ),
    val s2Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 18.sp
    ),
    val b1Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 16.sp
    ),
    val b1Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 16.sp
    ),
    val b1SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 16.sp
    ),
    val b1Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 16.sp
    ),
    val b2Regular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 14.sp
    ),
    val b2Medium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 14.sp
    ),
    val b2SemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 14.sp
    ),
    val b2Bold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 14.sp
    ),
    val overLineRegular: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Normal,
        fontSize = 12.sp
    ),
    val overLineMedium: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Medium,
        fontSize = 12.sp
    ),
    val overLineSemiBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.SemiBold,
        fontSize = 12.sp
    ),
    val overLineBold: TextStyle = TextStyle(
        fontFamily = fonts,
        fontWeight = FontWeight.Bold,
        fontSize = 12.sp
    ),
)

internal val LocalTypography = staticCompositionLocalOf { AdminTypography() }
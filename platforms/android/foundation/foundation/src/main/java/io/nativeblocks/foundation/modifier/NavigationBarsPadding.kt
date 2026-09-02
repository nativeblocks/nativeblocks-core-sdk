package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.navigationBarsPadding
import io.nativeblocks.compiler.type.Modifier
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/navigation_bars_padding",
    name = "Navigation Bars Padding",
    description = "Pads the block by the navigation bar window inset.",
    version = 1,
    versionName = "1",
)
internal fun navigationBarsPadding(): UiModifier = UiModifier.navigationBarsPadding()

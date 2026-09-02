package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.statusBarsPadding
import io.nativeblocks.compiler.type.Modifier
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/status_bars_padding",
    name = "Status Bars Padding",
    description = "Pads the block by the status bar window inset.",
    version = 1,
    versionName = "1",
)
internal fun statusBarsPadding(): UiModifier = UiModifier.statusBarsPadding()

package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.imePadding
import io.nativeblocks.compiler.type.Modifier
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/ime_padding",
    name = "Ime Padding",
    description = "Pads the block by the keyboard (IME) window inset while it is visible.",
    version = 1,
    versionName = "1",
)
internal fun imePadding(): UiModifier = UiModifier.imePadding()

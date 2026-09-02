package io.nativeblocks.foundation.modifier

import androidx.compose.animation.animateContentSize
import io.nativeblocks.compiler.type.Modifier
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/animate_size",
    name = "Animate Size",
    description = "Animates the block's size whenever its content size changes.",
    version = 1,
    versionName = "1",
)
internal fun animateSize(): UiModifier = UiModifier.animateContentSize()

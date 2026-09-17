package io.nativeblocks.foundation.modifier

import androidx.compose.animation.animateContentSize
import androidx.compose.animation.core.FiniteAnimationSpec
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.VisibilityThreshold
import androidx.compose.animation.core.snap
import androidx.compose.animation.core.spring
import androidx.compose.ui.unit.IntSize
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

private val ANIMATED_SIZE_SPEC: FiniteAnimationSpec<IntSize> =
    spring(stiffness = Spring.StiffnessMediumLow, visibilityThreshold = IntSize.VisibilityThreshold)
private val INSTANT_SIZE_SPEC: FiniteAnimationSpec<IntSize> = snap()

@Modifier(
    keyType = "nativeblocks/animate_size",
    name = "Animate Size",
    description = "Animates the block's size whenever its content size changes.",
    version = 1,
    versionName = "1",
)
internal fun animateSize(
    @ModifierData(
        description = "Whether size changes are animated. When false, they apply instantly.",
        defaultValue = "true",
    )
    enabled: Boolean = true,
): UiModifier = UiModifier.animateContentSize(
    animationSpec = if (enabled) ANIMATED_SIZE_SPEC else INSTANT_SIZE_SPEC,
)
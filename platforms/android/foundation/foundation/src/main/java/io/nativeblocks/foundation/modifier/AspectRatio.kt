package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.aspectRatio
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/aspect_ratio",
    name = "Aspect Ratio",
    description = "Sizes the block to a width/height ratio.",
    version = 1,
    versionName = "1",
)
internal fun aspectRatio(
    @ModifierData(
        description = "Width to height ratio (e.g. 1.777 for 16:9); must be greater than 0.",
        defaultValue = "1.0",
    ) ratio: Float = 1f,
    @ModifierData(description = "Whether height constraints are matched before width.", defaultValue = "false")
    matchHeightConstraintsFirst: Boolean = false,
): UiModifier {
    // Modifier.aspectRatio throws on non-positive ratios
    if (ratio <= 0f) return UiModifier
    return UiModifier.aspectRatio(ratio, matchHeightConstraintsFirst)
}

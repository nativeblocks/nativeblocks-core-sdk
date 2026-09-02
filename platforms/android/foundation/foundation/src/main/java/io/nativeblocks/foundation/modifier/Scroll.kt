package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

private const val DIRECTION_VERTICAL = "vertical"
private const val DIRECTION_HORIZONTAL = "horizontal"

@Modifier(
    keyType = "nativeblocks/scroll",
    name = "Scroll",
    description = "Makes the block scrollable in one direction.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun scroll(
    @ModifierData(description = "Scroll direction (vertical, horizontal).", defaultValue = "vertical")
    direction: String = DIRECTION_VERTICAL,
): UiModifier = when (direction) {
    DIRECTION_HORIZONTAL -> UiModifier.horizontalScroll(rememberScrollState())
    DIRECTION_VERTICAL -> UiModifier.verticalScroll(rememberScrollState())
    else -> UiModifier
}

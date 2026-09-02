package io.nativeblocks.foundation.modifier

import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier
import androidx.compose.ui.platform.testTag as uiTestTag

@Modifier(
    keyType = "nativeblocks/test_tag",
    name = "Test Tag",
    description = "Tags the block for UI tests (Maestro).",
    version = 1,
    versionName = "1",
)
internal fun testTag(
    @ModifierData(description = "The tag used by UI test selectors.")
    tag: String,
): UiModifier {
    if (tag.isBlank()) return UiModifier
    return UiModifier.uiTestTag(tag)
}

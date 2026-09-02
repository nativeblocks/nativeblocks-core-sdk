package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.RowScope
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/weight",
    name = "Weight",
    description = "Distributes remaining space of the parent column/row to the block.",
    version = 1,
    versionName = "1",
)
internal fun weight(
    modifierContext: ModifierContext,
    @ModifierData(description = "Proportional weight of the block; must be greater than 0.", defaultValue = "1.0")
    value: Float = 1f,
    @ModifierData(description = "Whether the block fills the space given by the weight.", defaultValue = "true")
    fill: Boolean = true,
): UiModifier {
    if (value <= 0f) return UiModifier
    // weight only exists on column/row scopes; outside them the modifier is a no-op
    return when (val scope = modifierContext.scope) {
        is ColumnScope -> with(scope) { UiModifier.weight(value, fill) }
        is RowScope -> with(scope) { UiModifier.weight(value, fill) }
        else -> UiModifier
    }
}

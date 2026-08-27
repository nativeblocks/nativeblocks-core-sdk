package io.nativeblocks.sample.modifier

import androidx.compose.foundation.layout.RowScope
import androidx.compose.ui.Modifier as ComposeModifier
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@Modifier(
    keyType = "nativeblocks/weight",
    name = "Weight",
    description = "Distributes the remaining space of a row between its children.",
    scope = "ROW",
)
fun NativeWeight(
    modifierContext: ModifierContext? = null,
    @ModifierData(
        description = "Share of the remaining space this block takes.",
        defaultValue = "1.0"
    ) weight: Float = 1f,
): ComposeModifier {
    val scope = modifierContext?.scope as? RowScope ?: return ComposeModifier
    return with(scope) { ComposeModifier.weight(weight) }
}

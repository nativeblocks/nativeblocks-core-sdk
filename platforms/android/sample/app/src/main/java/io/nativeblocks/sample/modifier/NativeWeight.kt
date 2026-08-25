package io.nativeblocks.sample.modifier

import androidx.compose.foundation.layout.RowScope
import androidx.compose.ui.Modifier
import io.nativeblocks.compiler.type.NativeModifier
import io.nativeblocks.compiler.type.NativeModifierData
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext

@NativeModifier(
    keyType = "nativeblocks/weight",
    name = "Weight",
    description = "Distributes the remaining space of a row between its children.",
    scope = "ROW",
)
fun NativeWeight(
    modifierContext: ModifierContext? = null,
    @NativeModifierData(
        description = "Share of the remaining space this block takes.",
        defaultValue = "1.0"
    ) weight: Float = 1f,
): Modifier {
    val scope = modifierContext?.scope as? RowScope ?: return Modifier
    return with(scope) { Modifier.weight(weight) }
}

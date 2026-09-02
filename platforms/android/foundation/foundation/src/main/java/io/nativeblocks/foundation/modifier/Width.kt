package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/width",
    name = "Width",
    description = "Sets the width of the block.",
    version = 1,
    versionName = "1",
)
internal fun width(
    @ModifierData(
        description = "The width of the block ('match', 'wrap' or a number in DP).",
        defaultValue = "wrap",
    ) value: String = "wrap",
): UiModifier = when (value) {
    "match" -> UiModifier.fillMaxWidth()
    "wrap" -> UiModifier.wrapContentWidth()
    else -> {
        val dpValue = value.toFloatOrNull()
        if (dpValue != null) UiModifier.width(dpValue.dp) else UiModifier
    }
}

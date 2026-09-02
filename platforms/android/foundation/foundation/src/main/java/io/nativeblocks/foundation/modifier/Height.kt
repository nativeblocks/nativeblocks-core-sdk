package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.wrapContentHeight
import androidx.compose.ui.unit.dp
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/height",
    name = "Height",
    description = "Sets the height of the block.",
    version = 1,
    versionName = "1",
)
internal fun height(
    @ModifierData(
        description = "The height of the block ('match', 'wrap' or a number in DP).",
        defaultValue = "wrap",
    ) value: String = "wrap",
): UiModifier = when (value) {
    "match" -> UiModifier.fillMaxHeight()
    "wrap" -> UiModifier.wrapContentHeight()
    else -> {
        val dpValue = value.toFloatOrNull()
        if (dpValue != null) UiModifier.height(dpValue.dp) else UiModifier
    }
}

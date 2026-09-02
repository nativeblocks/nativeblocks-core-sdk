package io.nativeblocks.foundation.modifier

import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.LocalIndication
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.hapticfeedback.HapticFeedbackType
import androidx.compose.ui.platform.LocalHapticFeedback
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.compiler.type.ModifierEvent
import androidx.compose.ui.Modifier as UiModifier

@OptIn(ExperimentalFoundationApi::class)
@Modifier(
    keyType = "nativeblocks/clickable",
    name = "Clickable",
    description = "Makes the block clickable (and optionally long-clickable).",
    version = 1,
    versionName = "1",
)
@Composable
internal fun clickable(
    @ModifierData(description = "Whether click handling is enabled.", defaultValue = "true")
    enabled: Boolean = true,
    @ModifierData(description = "Whether a ripple indication is shown on click.", defaultValue = "true")
    ripple: Boolean = true,
    @ModifierData(description = "Whether a haptic feedback fires on long click.", defaultValue = "true")
    hapticsOnLongClick: Boolean = true,
    @ModifierEvent(description = "Triggered when the block is clicked.")
    onClick: (() -> Unit)? = null,
    @ModifierEvent(description = "Triggered when the block is long-clicked.")
    onLongClick: (() -> Unit)? = null,
): UiModifier {
    if (onClick == null && onLongClick == null) return UiModifier
    val haptic = LocalHapticFeedback.current
    val interactionSource = remember { MutableInteractionSource() }
    val indication = if (ripple) LocalIndication.current else null
    return UiModifier.combinedClickable(
        interactionSource = interactionSource,
        indication = indication,
        enabled = enabled,
        onLongClick = onLongClick?.let { callback ->
            {
                if (hapticsOnLongClick) haptic.performHapticFeedback(HapticFeedbackType.LongPress)
                callback()
            }
        },
    ) {
        onClick?.invoke()
    }
}

package io.nativeblocks.foundation.modifier

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import io.nativeblocks.compiler.type.Modifier
import io.nativeblocks.compiler.type.ModifierData
import io.nativeblocks.compiler.type.ModifierEvent
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext
import androidx.compose.ui.Modifier as UiModifier

@Modifier(
    keyType = "nativeblocks/paginate",
    name = "Paginate",
    description = "Fires an event when the list nears its end; attach it to the content of a lazy row/column.",
    version = 1,
    versionName = "1",
)
@Composable
internal fun paginate(
    modifierContext: ModifierContext,
    @ModifierData(
        description = "Total number of items in the list; bind the same value as the list length.",
        defaultValue = "0",
    ) length: Int = 0,
    @ModifierData(
        description = "Current page; the event reports this value plus one.",
        defaultValue = "1",
    ) currentPage: Int = 1,
    @ModifierData(
        description = "Whether there are more pages to load.",
        defaultValue = "true",
    ) hasNextPage: Boolean = true,
    @ModifierData(
        description = "How many items before the end of the list the event fires.",
        defaultValue = "0",
    ) threshold: Int = 0,
    @ModifierEvent(
        description = "Fired when the end of the list is reached; reports the next page to fetch.",
        dataBindings = ["currentPage"],
    ) onNextPage: ((Int) -> Unit)? = null,
): UiModifier {
    val index = modifierContext.listItemIndex
    val reachedEnd = index != NONE_INDEX &&
        length > 0 &&
        index >= length - 1 - threshold.coerceAtLeast(0)
    LaunchedEffect(index, length, currentPage, hasNextPage, reachedEnd) {
        if (reachedEnd && hasNextPage) {
            onNextPage?.invoke(currentPage + 1)
        }
    }
    return UiModifier
}

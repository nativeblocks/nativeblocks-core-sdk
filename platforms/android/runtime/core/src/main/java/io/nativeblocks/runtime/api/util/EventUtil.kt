package io.nativeblocks.runtime.api.util

import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext
import io.nativeblocks.runtime.api.provider.model.NativeActionModel

private fun blockHandleEvent(blockContext: BlockContext, action: NativeActionModel, eventType: String) {
    blockContext.onHandleAction.invoke(
        blockContext.listItemIndex, action, eventType
    )
}

fun blockProvideEvent(
    blockContext: BlockContext,
    eventType: String
): (() -> Unit)? {
    val action = blockContext.onFindAction.invoke(eventType)
    return if (action != null) {
        { blockHandleEvent(blockContext, action, eventType) }
    } else {
        null
    }
}

private fun modifierHandleEvent(
    modifierContext: ModifierContext,
    action: NativeActionModel,
    eventType: String
) {
    modifierContext.onHandleAction.invoke(
        modifierContext.listItemIndex, action, eventType
    )
}

fun modifierProvideEvent(
    modifierContext: ModifierContext,
    eventType: String
): (() -> Unit)? {
    val action = modifierContext.onFindAction.invoke(eventType)
    return if (action != null) {
        { modifierHandleEvent(modifierContext, action, eventType) }
    } else {
        null
    }
}

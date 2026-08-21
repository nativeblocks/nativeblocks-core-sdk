package io.nativeblocks.runtime.api.util

import io.nativeblocks.runtime.api.provider.block.BlockContext
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

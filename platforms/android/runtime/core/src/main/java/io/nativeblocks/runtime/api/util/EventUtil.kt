package io.nativeblocks.runtime.api.util

import io.nativeblocks.runtime.api.provider.block.BlockProps
import io.nativeblocks.runtime.api.provider.model.NativeActionModel

private fun blockHandleEvent(blockProps: BlockProps, action: NativeActionModel, eventType: String) {
    blockProps.onHandleAction.invoke(
        blockProps.listItemIndex, action, eventType
    )
}

fun blockProvideEvent(
    blockProps: BlockProps,
    eventType: String
): (() -> Unit)? {
    val action = blockProps.onFindAction.invoke(eventType)
    return if (action != null) {
        { blockHandleEvent(blockProps, action, eventType) }
    } else {
        null
    }
}

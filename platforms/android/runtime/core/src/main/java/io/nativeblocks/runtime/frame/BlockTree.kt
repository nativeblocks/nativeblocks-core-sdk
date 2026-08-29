@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.frame

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.nativeblocks.runtime.api.provider.block.BlockContext
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import io.nativeblocks.runtime.api.provider.block.defaults.InternalFallbackBlock
import io.nativeblocks.runtime.api.provider.block.defaults.RootBlock
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.modifier.ModifierContext
import io.nativeblocks.runtime.api.util.LocalNativeWindowWidthClass
import io.nativeblocks.runtime.api.util.currentWindowWidthClass
import io.nativeblocks.runtime.ffi.RenderingState

@Composable
internal fun NativeFrame(
    instanceName: String,
    frameViewModel: FrameViewModel,
    loading: @Composable () -> Unit,
    error: @Composable (String) -> Unit
) {
    val state by frameViewModel.renderingState.collectAsStateWithLifecycle()
    CompositionLocalProvider(LocalNativeWindowWidthClass provides currentWindowWidthClass()) {
        when (val frameState = state) {
            is RenderingState.Loading -> loading.invoke()
            is RenderingState.Ready -> RootLifecycle(instanceName, frameViewModel)
            is RenderingState.Error -> error.invoke(frameState.message)
        }
    }
}

@Composable
private fun RootLifecycle(instanceName: String, vm: FrameViewModel) {
    val rootKeyState by vm.rootKey.collectAsStateWithLifecycle()
    val appearGen by vm.frameUpdateGeneration.collectAsStateWithLifecycle()

    val rootKey = rootKeyState ?: return

    DisposableEffect(rootKey) {
        onDispose { vm.rootExited(rootKey) }
    }

    LaunchedEffect(appearGen) {
        vm.rootEntered(rootKey)
    }

    Block(instanceName, vm, rootKey, NONE_INDEX, appearGen, parentScope = null)
}

@Composable
private fun Block(
    instanceName: String,
    vm: FrameViewModel,
    blockKey: String,
    listItemIndex: Int,
    frameGeneration: Int,
    parentScope: Any?,
) {
    val block = vm.blockOf(blockKey) ?: return
    if (vm.valueOf(block.visibility) == "false") return

    val nativeBlock = remember(block.keyType) {
        if (block.keyType == "ROOT") { blockContext -> RootBlock(blockContext) }
        else vm.blockProvider.getProvidedBlocks()[block.keyType]
    }
    if (nativeBlock == null) {
        vm.logBlockFallback(block.keyType, blockKey)
        vm.blockProvider.getFallbackBlock()?.invoke(block.keyType, blockKey) ?: InternalFallbackBlock(block.keyType)
        return
    }

    val modifier = blockModifier(instanceName, vm, block, blockKey, listItemIndex, parentScope)

    val blockContext = remember(block, listItemIndex, modifier) {
        BlockContext(
            instanceName = instanceName,
            listItemIndex = listItemIndex,
            onFindVariable = { data -> vm.valueOf(data?.value.orEmpty()) },
            onUpdateVariable = { data, value -> vm.updateVariable(data?.value.orEmpty(), value) },
            onFindAction = { vm.actionOf(blockKey, it) },
            onHandleAction = { index, action, event -> vm.handleAction(index, action, event) },
            block = block,
            modifier = modifier,
            onSubBlock = { blockKeys, subSlot, itemIndex, scope ->
                blockKeys[subSlot.slot]?.forEach { childKey ->
                    key(childKey) {
                        Block(
                            instanceName = instanceName,
                            vm = vm,
                            blockKey = childKey,
                            listItemIndex = if (itemIndex == NONE_INDEX) listItemIndex else itemIndex,
                            frameGeneration = frameGeneration,
                            parentScope = scope,
                        )
                    }
                }
            },
        )
    }

    nativeBlock.invoke(blockContext)
}

@Composable
private fun blockModifier(
    instanceName: String,
    vm: FrameViewModel,
    block: NativeBlockModel,
    blockKey: String,
    listItemIndex: Int,
    parentScope: Any?,
): Modifier {
    if (block.modifiers.isEmpty()) {
        return Modifier
    }
    val provided = vm.modifierProvider.getProvidedModifiers()
    var chain: Modifier = Modifier
    block.modifiers.forEach { item ->
        val nativeModifier = provided[item.keyType]
        if (nativeModifier != null) {
            val modifierContext = ModifierContext(
                instanceName = instanceName,
                listItemIndex = listItemIndex,
                onFindVariable = { data -> vm.valueOf(data?.value.orEmpty()) },
                onUpdateVariable = { data, value -> vm.updateVariable(data?.value.orEmpty(), value) },
                onFindAction = { vm.actionOf(blockKey, it) },
                onHandleAction = { index, action, event -> vm.handleAction(index, action, event) },
                modifier = item,
                scope = parentScope,
            )
            chain = chain.then(key(item.keyType, item.position) { nativeModifier.invoke(modifierContext) })
        } else {
            vm.logModifierFallback(item.keyType, blockKey)
        }
    }
    return chain
}

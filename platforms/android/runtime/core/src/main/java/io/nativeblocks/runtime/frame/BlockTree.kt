package io.nativeblocks.runtime.frame

import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.remember
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import io.nativeblocks.runtime.api.provider.block.BlockProps
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import io.nativeblocks.runtime.api.provider.block.defaults.InternalFallbackBlock
import io.nativeblocks.runtime.api.provider.block.defaults.RootBlock
import io.nativeblocks.runtime.api.util.LocalNativeWindowWidthClass
import io.nativeblocks.runtime.api.util.currentWindowWidthClass
import io.nativeblocks.runtime.ffi.RenderingState

private const val ON_APPEAR = "onAppear"
private const val ON_DISAPPEAR = "onDisappear"

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

    DisposableEffect(appearGen) {
        val onAppearAction = vm.actionOf(rootKey, ON_APPEAR)
        vm.handleAction(NONE_INDEX, onAppearAction, ON_APPEAR)
        onDispose {
            val onDisappearAction = vm.actionOf(rootKey, ON_DISAPPEAR)
            vm.handleAction(NONE_INDEX, onDisappearAction, ON_DISAPPEAR)
        }
    }

    Block(instanceName, vm, rootKey, NONE_INDEX)
}

@Composable
private fun Block(instanceName: String, vm: FrameViewModel, blockKey: String, listItemIndex: Int) {
    val block = vm.blockOf(blockKey)?.value ?: return

    val nativeBlock = remember(block.keyType) {
        if (block.keyType == "ROOT") { props -> RootBlock(props) }
        else vm.blockProvider.getProvidedBlocks()[block.keyType]
    }
    if (nativeBlock == null) {
        vm.blockProvider.getFallbackBlock()?.invoke(block.keyType, blockKey) ?: InternalFallbackBlock(block.keyType)
        return
    }

    val props = remember(block, listItemIndex) {
        BlockProps(
            instanceName = instanceName,
            listItemIndex = listItemIndex,
            onFindVariable = { vm.variableOf(it)?.value },
            onVariableChange = { vm.updateVariable(it.key, it.value) },
            onFindAction = { vm.actionOf(blockKey, it) },
            onHandleAction = { index, action, event -> vm.handleAction(index, action, event) },
            block = block,
            onSubBlock = { blockKeys, subSlot, itemIndex, _ ->
                blockKeys[subSlot.slot]?.forEach { childKey ->
                    key(childKey) {
                        Block(
                            instanceName = instanceName,
                            vm = vm,
                            blockKey = childKey,
                            listItemIndex = if (itemIndex == NONE_INDEX) listItemIndex else itemIndex,
                        )
                    }
                }
            },
        )
    }

    nativeBlock.invoke(props)
}

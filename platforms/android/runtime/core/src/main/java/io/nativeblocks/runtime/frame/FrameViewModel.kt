package io.nativeblocks.runtime.frame

import androidx.compose.runtime.MutableState
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.snapshots.Snapshot
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.nativeblocks.runtime.api.provider.block.NativeBlockProviderRegistry
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import io.nativeblocks.runtime.ffi.FrameDiff
import io.nativeblocks.runtime.ffi.FrameFull
import io.nativeblocks.runtime.ffi.RenderingState
import io.nativeblocks.runtime.ffi.toDomain
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import java.util.concurrent.ConcurrentHashMap
import io.nativeblocks.runtime.ffi.NativeBlockModel as RuntimeFFIBlockModel
import io.nativeblocks.runtime.ffi.NativeVariableModel as RuntimeFFIVariableModel

internal class FrameViewModel(
    frameStateBridge: FrameStateBridge,
    private val instanceName: String,
) : ViewModel(),
    FrameStateBridge by frameStateBridge {

    private val _renderingState = MutableStateFlow<RenderingState>(RenderingState.Loading)
    val renderingState: StateFlow<RenderingState> = _renderingState.asStateFlow()

    private val _rootKey = MutableStateFlow<String?>(null)
    val rootKey: StateFlow<String?> = _rootKey.asStateFlow()

    private val _frameUpdateGeneration = MutableStateFlow(0)
    val frameUpdateGeneration = _frameUpdateGeneration.asStateFlow()

    private val blocks = ConcurrentHashMap<String, MutableState<NativeBlockModel>>()
    private val variables = ConcurrentHashMap<String, MutableState<NativeVariableModel>>()
    private val actions = ConcurrentHashMap<String, List<NativeActionModel>>()

    val blockProvider = NativeBlockProviderRegistry.getOrCreate(instanceName)

    private val actionTree = ActionTree(
        instanceName = instanceName,
        coroutineScope = viewModelScope,
        onFindVariable = { key ->
            variableOf(key)?.value
        },
        onFindBlock = { key ->
            blockOf(key)?.value
        },
        onChangeBlock = { blockKey, propertyKey, valueMobile, valueTablet, valueDesktop ->
            updateBlockProperty(blockKey, propertyKey, valueMobile, valueTablet, valueDesktop)
        },
        onVariableChange = { variableModel ->
            updateVariable(variableModel.key, variableModel.value)
        }
    )

    fun setupFrame(route: String, routeArguments: Map<String, String>) {
        viewModelScope.launch(Dispatchers.IO) {
            observeFrame(
                route = route,
                args = routeArguments,
                onFull = {
                    viewModelScope.launch(Dispatchers.IO) { applyFull(it) }
                },
                onDiff = {
                    viewModelScope.launch(Dispatchers.IO) { applyDiff(it) }
                }
            )
        }
    }

    fun variableOf(key: String): State<NativeVariableModel>? {
        return variables[key]
    }

    fun blockOf(key: String): State<NativeBlockModel>? {
        return blocks[key]
    }

    fun actionOf(blockKey: String, eventType: String): NativeActionModel? {
        return actions[blockKey]?.firstOrNull { it.event == eventType }
    }

    fun handleAction(index: Int, action: NativeActionModel?, performedEventType: String) {
        actionTree.handle(index, action, performedEventType)
    }

    private fun applyFull(frame: FrameFull) {
        Snapshot.withMutableSnapshot {
            variables.keys.retainAll(frame.variables.keys)
            syncVariables(frame.variables)
            blocks.keys.retainAll(frame.blocks.keys)
            syncBlocks(frame.blocks)
        }
        actions.clear()
        actions.putAll(frame.actions.mapValues { entry ->
            entry.value.map { it.toDomain() }
        })

        _rootKey.update { frame.rootKey }
        if (frame.state is RenderingState.Ready) {
            _frameUpdateGeneration.update { it + 1 }
        }
        _renderingState.update { frame.state }
    }

    private fun applyDiff(diff: FrameDiff) {
        Snapshot.withMutableSnapshot {
            if (diff.variables.isNotEmpty()) {
                syncVariables(diff.variables)
            }
            if (diff.blocks.isNotEmpty()) {
                syncBlocks(diff.blocks)
            }
        }
    }

    private fun syncVariables(runtimeVariables: Map<String, RuntimeFFIVariableModel>) {
        runtimeVariables.forEach { (key, variable) ->
            val domain = variable.toDomain()
            val cell = variables[key]
            if (cell == null) variables[key] = mutableStateOf(domain) else cell.value = domain
        }
    }

    private fun syncBlocks(runtimeBlocks: Map<String, RuntimeFFIBlockModel>) {
        runtimeBlocks.forEach { (key, block) ->
            val domain = block.toDomain()
            val cell = blocks[key]
            if (cell == null) blocks[key] = mutableStateOf(domain) else cell.value = domain
        }
    }

    override fun onCleared() {
        releaseFrame()
        super.onCleared()
    }
}
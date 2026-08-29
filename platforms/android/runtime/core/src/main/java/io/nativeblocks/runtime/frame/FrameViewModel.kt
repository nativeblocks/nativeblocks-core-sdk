@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.frame

import androidx.compose.runtime.MutableState
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.snapshots.Snapshot
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import io.nativeblocks.runtime.api.provider.block.NONE_INDEX
import io.nativeblocks.runtime.api.provider.block.NativeBlockProviderRegistry
import io.nativeblocks.runtime.api.provider.modifier.NativeModifierProviderRegistry
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import io.nativeblocks.runtime.ffi.ActionLogEvent
import io.nativeblocks.runtime.ffi.BlockLogEvent
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
import io.nativeblocks.runtime.ffi.NativeActionModel as RuntimeFFIActionModel
import io.nativeblocks.runtime.ffi.NativeBlockModel as RuntimeFFIBlockModel
import io.nativeblocks.runtime.ffi.NativeVariableModel as RuntimeFFIVariableModel

private const val ON_APPEAR = "onAppear"
private const val ON_DISAPPEAR = "onDisappear"
private const val ON_STATE_SAVED = "onStateSaved"
private const val ON_STATE_RESTORED = "onStateRestored"

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

    private var announcedGeneration = -1
    private var isAppeared = false
    private var isRestored = false
    private var isStateful = false

    private val blocks = ConcurrentHashMap<String, NativeBlockModel>()
    private val variables = ConcurrentHashMap<String, MutableState<NativeVariableModel>>()
    private val actions = ConcurrentHashMap<String, List<NativeActionModel>>()

    val blockProvider = NativeBlockProviderRegistry.getOrCreate(instanceName)
    val modifierProvider = NativeModifierProviderRegistry.getOrCreate(instanceName)

    private val actionTree = ActionTree(
        instanceName = instanceName,
        coroutineScope = viewModelScope,
        onFindVariable = { key ->
            variableOf(key)?.value
        },
        onVariableChange = { variableModel ->
            updateVariable(variableModel.key, variableModel.value)
        },
        onLog = { event -> logAction(event) }
    )

    fun setupFrame(route: String, routeArguments: Map<String, String>, stateKey: String? = null) {
        isStateful = stateKey != null
        viewModelScope.launch(Dispatchers.IO) {
            observeFrame(
                route = route,
                args = routeArguments,
                stateKey = stateKey,
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

    fun valueOf(key: String): String? = variableOf(key)?.value?.value

    fun blockOf(key: String): NativeBlockModel? {
        return blocks[key]
    }

    fun actionOf(blockKey: String, eventType: String): NativeActionModel? {
        val matches = actions[blockKey]?.filter { it.event == eventType }.orEmpty()
        if (matches.size > 1) {
            logAction(ActionLogEvent.EventAmbiguous(eventType, blockKey, matches.size))
        }
        return matches.firstOrNull()
    }

    fun logBlockFallback(keyType: String, blockKey: String) {
        logBlock(BlockLogEvent.BlockFallback(keyType = keyType, blockKey = blockKey))
    }

    fun logModifierFallback(keyType: String, blockKey: String) {
        logBlock(BlockLogEvent.ModifierFallback(keyType = keyType, blockKey = blockKey))
    }

    fun handleAction(index: Int, action: NativeActionModel?, performedEventType: String) {
        actionTree.handle(index, action, performedEventType)
    }

    fun rootEntered(rootKey: String) {
        isAppeared = true
        val generation = _frameUpdateGeneration.value
        if (announcedGeneration == generation) return
        announcedGeneration = generation
        val event = if (isRestored) ON_STATE_RESTORED else ON_APPEAR
        handleAction(NONE_INDEX, actionOf(rootKey, event), event)
    }

    fun rootExited(rootKey: String) {
        if (!isAppeared) return
        isAppeared = false
        val event = if (isStateful) ON_STATE_SAVED else ON_DISAPPEAR
        handleAction(NONE_INDEX, actionOf(rootKey, event), event)
    }

    private fun applyFull(frame: FrameFull) {
        Snapshot.withMutableSnapshot {
            variables.keys.retainAll(frame.variables.keys)
            syncVariables(frame.variables)
        }
        syncBlocks(frame.blocks)
        syncActions(frame.actions)

        _rootKey.update { frame.rootKey }
        isRestored = frame.restored
        if (frame.state is RenderingState.Ready) {
            _frameUpdateGeneration.update { it + 1 }
        }
        _renderingState.update { frame.state }
    }

    private fun applyDiff(diff: FrameDiff) {
        if (diff.variables.isEmpty()) return
        Snapshot.withMutableSnapshot {
            syncVariables(diff.variables)
        }
    }

    private fun syncVariables(runtimeVariables: Map<String, RuntimeFFIVariableModel>) {
        runtimeVariables.forEach { (key, variable) ->
            val domain = variable.toDomain()
            val cell = variables[key]
            if (cell == null) variables[key] = mutableStateOf(domain) else cell.value = domain
        }
    }

    private fun syncActions(runtimeActions: Map<String, List<RuntimeFFIActionModel>>) {
        actions.clear()
        actions.putAll(runtimeActions.mapValues { entry -> entry.value.map { it.toDomain() } })
    }

    private fun syncBlocks(runtimeBlocks: Map<String, RuntimeFFIBlockModel>) {
        blocks.clear()
        runtimeBlocks.forEach { (key, block) -> blocks[key] = block.toDomain() }
    }

    override fun onCleared() {
        releaseFrame()
        super.onCleared()
    }
}
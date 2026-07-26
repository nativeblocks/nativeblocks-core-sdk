package io.nativeblocks.runtime.frame

import io.nativeblocks.runtime.api.provider.action.ActionProps
import io.nativeblocks.runtime.api.provider.action.INativeAction
import io.nativeblocks.runtime.api.provider.action.NativeActionProviderRegistry
import io.nativeblocks.runtime.api.provider.action.defaults.NativeScriptAction
import io.nativeblocks.runtime.api.provider.action.defaults.internalFallbackAction
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerThen
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import kotlinx.coroutines.CoroutineScope

internal class ActionTree(
    private val instanceName: String,
    private val coroutineScope: CoroutineScope,
    private val onFindVariable: (String) -> NativeVariableModel?,
    private val onFindBlock: (String) -> NativeBlockModel?,
    private val onChangeBlock: (String, String, String, String, String) -> Unit,
    private val onVariableChange: (NativeVariableModel) -> Unit,
) {

    private val nativeActionProvider get() = NativeActionProviderRegistry.getOrCreate(instanceName)

    fun handle(index: Int, action: NativeActionModel?, performedEventType: String) {
        if (action == null || action.event != performedEventType) return

        action.triggers
            .filter { it.parentId.isEmpty() }
            .forEach { trigger ->
                handleTrigger(
                    action = action,
                    index = index,
                    trigger = trigger.copy(subTriggers = action.triggers.filter { it.parentId == trigger.id }),
                    onFind = { keyType ->
                        nativeActionProvider.getProvidedActions()[keyType]
                    },
                    onTriggerFallBack = { keyType, name ->
                        nativeActionProvider.getFallbackAction()?.invoke(keyType, name)
                            ?: internalFallbackAction(instanceName = instanceName, name = name)
                    }
                )
            }
    }

    private fun handleTrigger(
        action: NativeActionModel,
        index: Int,
        trigger: NativeActionTriggerModel,
        onFind: (String) -> INativeAction?,
        onTriggerFallBack: (keyType: String, name: String) -> Unit,
    ) {
        val nativeAction = if (trigger.keyType == NativeScriptAction.KEY_TYPE) {
            NativeScriptAction()
        } else {
            onFind(trigger.keyType)
        }

        if (nativeAction == null) {
            onTriggerFallBack(trigger.keyType, trigger.name)
            return
        }

        val actionProps = ActionProps(
            instanceName = instanceName,
            listItemIndex = index,
            coroutineScope = coroutineScope,
            trigger = trigger,
            onFindVariable = onFindVariable,
            onFindBlock = onFindBlock,
            onChangeBlockProperties = onChangeBlock,
            onChangeVariable = { variable ->
                variable?.let { onVariableChange(it) }
            },
            onHandleNextTrigger = {
                advanceSubTriggers(action, index, trigger, NativeActionTriggerThen.NEXT, onFind, onTriggerFallBack)
            },
            onHandleSuccessNextTrigger = {
                advanceSubTriggers(action, index, trigger, NativeActionTriggerThen.SUCCESS, onFind, onTriggerFallBack)
            },
            onHandleFailureNextTrigger = {
                advanceSubTriggers(action, index, trigger, NativeActionTriggerThen.FAILURE, onFind, onTriggerFallBack)
            }
        )
        nativeAction.handle(actionProps)
    }

    private fun advanceSubTriggers(
        action: NativeActionModel,
        index: Int,
        trigger: NativeActionTriggerModel,
        then: NativeActionTriggerThen,
        onFind: (String) -> INativeAction?,
        onTriggerFallBack: (keyType: String, name: String) -> Unit,
    ) {
        action.triggers
            .filter { it.parentId == trigger.id && it.then == then }
            .forEach { handleTrigger(action, index, it, onFind, onTriggerFallBack) }
    }
}

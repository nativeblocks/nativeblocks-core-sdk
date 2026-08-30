@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.frame

import io.nativeblocks.runtime.api.provider.action.ActionContext
import io.nativeblocks.runtime.api.provider.action.INativeAction
import io.nativeblocks.runtime.api.provider.action.NativeActionProviderRegistry
import io.nativeblocks.runtime.api.provider.action.defaults.NativeScriptAction
import io.nativeblocks.runtime.api.provider.action.defaults.internalFallbackAction
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import io.nativeblocks.runtime.api.provider.model.resolveIn
import io.nativeblocks.runtime.ffi.ActionLogEvent
import kotlinx.coroutines.CoroutineScope

internal class ActionTree(
    private val instanceName: String,
    private val coroutineScope: CoroutineScope,
    private val onFindVariable: (String) -> NativeVariableModel?,
    private val onVariableChange: (NativeVariableModel) -> Unit,
    private val onLog: (ActionLogEvent) -> Unit,
) {

    private val nativeActionProvider = NativeActionProviderRegistry.getOrCreate(instanceName)

    fun handle(index: Int, action: NativeActionModel?, performedEventType: String, scope: Any? = null) {
        if (action == null || action.event != performedEventType) {
            onLog(ActionLogEvent.EventIgnored(performedEventType))
            return
        }

        onLog(ActionLogEvent.EventTriggered(performedEventType, action.key))

        action.triggers
            .filter { it.parentId.isEmpty() }
            .forEach { trigger ->
                handleTrigger(
                    action = action,
                    index = index,
                    scope = scope,
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
        scope: Any?,
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
            onLog(ActionLogEvent.TriggerFallback(trigger.keyType, trigger.name))
            onTriggerFallBack(trigger.keyType, trigger.name)
            return
        }

        onLog(ActionLogEvent.TriggerExecuted(trigger.name, trigger.keyType, trigger.event))

        val actionContext = ActionContext(
            instanceName = instanceName,
            listItemIndex = index,
            coroutineScope = coroutineScope,
            trigger = trigger,
            onFindVariable = onFindVariable,
            onUpdateVariable = { variable ->
                variable?.let { onVariableChange(it) }
            },
            onHandleEvent = { event ->
                advanceSubTriggers(action, index, scope, trigger, event, onFind, onTriggerFallBack)
            },
            resolveTemplate = { value ->
                resolveIn(scope, value)
            },
        )
        nativeAction.handle(actionContext)
    }

    private fun advanceSubTriggers(
        action: NativeActionModel,
        index: Int,
        scope: Any?,
        trigger: NativeActionTriggerModel,
        event: String,
        onFind: (String) -> INativeAction?,
        onTriggerFallBack: (keyType: String, name: String) -> Unit,
    ) {
        action.triggers
            .filter { it.parentId == trigger.id && it.event == event }
            .forEach { handleTrigger(action, index, scope, it, onFind, onTriggerFallBack) }
    }
}

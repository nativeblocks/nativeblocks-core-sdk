import Foundation
import NativeblocksRuntimeFFI

internal final class ActionTree {

    private let instanceName: String
    private let onFindVariable: (String) -> NativeVariableModel?
    private let onVariableChange: (NativeVariableModel) -> Void
    private let onLog: (ActionLogEvent) -> Void

    init(
        instanceName: String,
        onFindVariable: @escaping (String) -> NativeVariableModel?,
        onVariableChange: @escaping (NativeVariableModel) -> Void,
        onLog: @escaping (ActionLogEvent) -> Void
    ) {
        self.instanceName = instanceName
        self.onFindVariable = onFindVariable
        self.onVariableChange = onVariableChange
        self.onLog = onLog
    }

    private var nativeActionProvider: NativeActionProvider {
        return NativeActionProviderRegistry.getOrCreate(instanceName)
    }

    func handle(index: Int, action: NativeActionModel?, performedEventType: String, scope: Any? = nil) {
        guard let action, action.event == performedEventType else {
            onLog(.eventIgnored(event: performedEventType))
            return
        }

        onLog(.eventTriggered(event: performedEventType, actionKey: action.key))

        for trigger in action.triggers where trigger.parentId.isEmpty {
            var rootTrigger = trigger
            rootTrigger.subTriggers = action.triggers.filter { $0.parentId == trigger.id }
            handleTrigger(
                action: action,
                index: index,
                scope: scope,
                trigger: rootTrigger,
                onFind: { [weak self] keyType in
                    self?.nativeActionProvider.getProvidedActions()[keyType]
                },
                onTriggerFallBack: { [weak self] keyType, name in
                    guard let self else { return }
                    if let fallback = self.nativeActionProvider.getFallbackAction() {
                        fallback(keyType, name)
                    } else {
                        internalFallbackAction(instanceName: self.instanceName, name: name)
                    }
                }
            )
        }
    }

    private func handleTrigger(
        action: NativeActionModel,
        index: Int,
        scope: Any?,
        trigger: NativeActionTriggerModel,
        onFind: @escaping (String) -> (any INativeAction)?,
        onTriggerFallBack: @escaping (String, String) -> Void
    ) {
        let nativeAction: (any INativeAction)? =
            trigger.keyType == NativeScriptAction.KEY_TYPE ? NativeScriptAction() : onFind(trigger.keyType)

        guard let nativeAction else {
            onLog(.triggerFallback(keyType: trigger.keyType, name: trigger.name))
            onTriggerFallBack(trigger.keyType, trigger.name)
            return
        }

        onLog(
            .triggerExecuted(
                name: trigger.name,
                keyType: trigger.keyType,
                event: trigger.event
            )
        )

        let actionContext = ActionContext(
            instanceName: instanceName,
            listItemIndex: index,
            onFindVariable: onFindVariable,
            onUpdateVariable: { [weak self] variable in
                guard let variable else { return }
                self?.onVariableChange(variable)
            },
            trigger: trigger,
            onHandleEvent: { [weak self] event in
                self?.advanceSubTriggers(
                    action: action, index: index, scope: scope, trigger: trigger, event: event,
                    onFind: onFind, onTriggerFallBack: onTriggerFallBack
                )
            },
            resolveTemplate: { resolveIn(scope, $0) }
        )
        nativeAction.handle(actionContext: actionContext)
    }

    private func advanceSubTriggers(
        action: NativeActionModel,
        index: Int,
        scope: Any?,
        trigger: NativeActionTriggerModel,
        event: String,
        onFind: @escaping (String) -> (any INativeAction)?,
        onTriggerFallBack: @escaping (String, String) -> Void
    ) {
        for subTrigger in action.triggers where subTrigger.parentId == trigger.id && subTrigger.event == event {
            handleTrigger(
                action: action,
                index: index,
                scope: scope,
                trigger: subTrigger,
                onFind: onFind,
                onTriggerFallBack: onTriggerFallBack
            )
        }
    }
}

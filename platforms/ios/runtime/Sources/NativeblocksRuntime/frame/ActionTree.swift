import Foundation
import NativeblocksRuntimeFFI

internal final class ActionTree {

    private let instanceName: String
    private let onFindVariable: (String) -> NativeVariableModel?
    private let onFindBlock: (String) -> NativeBlockModel?
    private let onChangeBlock: (String, String, String, String, String) -> Void
    private let onVariableChange: (NativeVariableModel) -> Void
    private let onLog: (ActionLogEvent) -> Void

    init(
        instanceName: String,
        onFindVariable: @escaping (String) -> NativeVariableModel?,
        onFindBlock: @escaping (String) -> NativeBlockModel?,
        onChangeBlock: @escaping (String, String, String, String, String) -> Void,
        onVariableChange: @escaping (NativeVariableModel) -> Void,
        onLog: @escaping (ActionLogEvent) -> Void
    ) {
        self.instanceName = instanceName
        self.onFindVariable = onFindVariable
        self.onFindBlock = onFindBlock
        self.onChangeBlock = onChangeBlock
        self.onVariableChange = onVariableChange
        self.onLog = onLog
    }

    private var nativeActionProvider: NativeActionProvider {
        return NativeActionProviderRegistry.getOrCreate(instanceName)
    }

    func handle(index: Int, action: NativeActionModel?, performedEventType: String) {
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
                then: String(describing: trigger.then)
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
            onFindBlock: onFindBlock,
            onUpdateBlockProperties: onChangeBlock,
            trigger: trigger,
            onHandleNextTrigger: { [weak self] _ in
                self?.advanceSubTriggers(
                    action: action, index: index, trigger: trigger, then: .next,
                    onFind: onFind, onTriggerFallBack: onTriggerFallBack
                )
            },
            onHandleSuccessNextTrigger: { [weak self] _ in
                self?.advanceSubTriggers(
                    action: action, index: index, trigger: trigger, then: .success,
                    onFind: onFind, onTriggerFallBack: onTriggerFallBack
                )
            },
            onHandleFailureNextTrigger: { [weak self] _ in
                self?.advanceSubTriggers(
                    action: action, index: index, trigger: trigger, then: .failure,
                    onFind: onFind, onTriggerFallBack: onTriggerFallBack
                )
            }
        )
        nativeAction.handle(actionContext: actionContext)
    }

    private func advanceSubTriggers(
        action: NativeActionModel,
        index: Int,
        trigger: NativeActionTriggerModel,
        then: NativeActionTriggerThen,
        onFind: @escaping (String) -> (any INativeAction)?,
        onTriggerFallBack: @escaping (String, String) -> Void
    ) {
        for subTrigger in action.triggers where subTrigger.parentId == trigger.id && subTrigger.then == then {
            handleTrigger(
                action: action,
                index: index,
                trigger: subTrigger,
                onFind: onFind,
                onTriggerFallBack: onTriggerFallBack
            )
        }
    }
}

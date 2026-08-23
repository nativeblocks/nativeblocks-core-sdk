import Foundation
import NativeblocksRuntimeFFI

internal typealias RuntimeFFIActionModel = NativeblocksRuntimeFFI.NativeActionModel
internal typealias RuntimeFFIActionTriggerModel = NativeblocksRuntimeFFI.NativeActionTriggerModel
internal typealias RuntimeFFIBlockModel = NativeblocksRuntimeFFI.NativeBlockModel
internal typealias RuntimeFFIVariableModel = NativeblocksRuntimeFFI.NativeVariableModel
internal typealias RuntimeFFILoggerEventLevel = NativeblocksRuntimeFFI.LoggerEventLevel

extension RuntimeFFILoggerEventLevel {
    internal func toDomain() -> LoggerEventLevel {
        switch self {
        case .debug: return .DEBUG
        case .error: return .ERROR
        }
    }
}

extension RuntimeFFIVariableModel {
    internal func toDomain() -> NativeVariableModel {
        return NativeVariableModel(
            key: key,
            value: value,
            type: variableType
        )
    }
}

extension RuntimeFFIBlockModel {
    internal func toDomain() -> NativeBlockModel {
        return NativeBlockModel(
            id: id,
            parentId: parentId,
            parentKey: parentKey,
            version: Int(version),
            slot: slot,
            keyType: keyType,
            key: key,
            scope: scope,
            visibility: visibility,
            position: Int(position),
            properties: properties.mapValues { property in
                NativeBlockPropertyModel(
                    key: property.key,
                    valueMobile: property.valueMobile,
                    valueTablet: property.valueTablet,
                    valueDesktop: property.valueDesktop,
                    type: property.propertyType
                )
            },
            data: data.mapValues { data in
                NativeBlockDataModel(key: data.key, value: data.value, type: data.dataType)
            },
            slots: slots.mapValues { slot in
                NativeBlockSlotModel(slot: slot.slot, scope: slot.scope)
            },
            subBlocks: subKeys
        )
    }
}

extension RuntimeFFIActionModel {
    internal func toDomain() -> NativeActionModel {
        return NativeActionModel(
            id: id,
            key: key,
            event: event,
            triggers: triggers.map { $0.toDomain() }
        )
    }
}

extension RuntimeFFIActionTriggerModel {
    fileprivate func toDomain() -> NativeActionTriggerModel {
        return NativeActionTriggerModel(
            id: id,
            parentId: parentId,
            version: Int(version),
            keyType: keyType,
            name: name,
            event: event,
            scope: scope,
            properties: properties.mapValues { property in
                NativeActionTriggerPropertyModel(
                    key: property.key,
                    value: property.value,
                    type: property.propertyType
                )
            },
            data: data.mapValues { data in
                NativeActionTriggerDataModel(key: data.key, value: data.value, type: data.dataType)
            },
            events: events.mapValues { event in
                NativeActionTriggerEventModel(event: event.event, scope: event.scope)
            }
        )
    }
}


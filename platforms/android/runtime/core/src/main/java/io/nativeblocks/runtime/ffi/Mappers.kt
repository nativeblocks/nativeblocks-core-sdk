@file:Suppress("DEPRECATION")

package io.nativeblocks.runtime.ffi

import io.nativeblocks.runtime.api.provider.logger.LoggerEventLevel
import io.nativeblocks.runtime.api.provider.model.NativeActionModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerDataModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerEventModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerModel
import io.nativeblocks.runtime.api.provider.model.NativeActionTriggerPropertyModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockDataModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockPropertyModel
import io.nativeblocks.runtime.api.provider.model.NativeBlockSlotModel
import io.nativeblocks.runtime.api.provider.model.NativeVariableModel
import io.nativeblocks.runtime.ffi.LoggerEventLevel as RuntimeFFILoggerEventLevel
import io.nativeblocks.runtime.ffi.NativeActionModel as RuntimeFFIActionModel
import io.nativeblocks.runtime.ffi.NativeActionTriggerModel as RuntimeFFIActionTriggerModel
import io.nativeblocks.runtime.ffi.NativeBlockModel as RuntimeFFIBlockModel
import io.nativeblocks.runtime.ffi.NativeVariableModel as RuntimeFFIVariableModel

internal fun RuntimeFFILoggerEventLevel.toDomain(): LoggerEventLevel {
    return when (this) {
        RuntimeFFILoggerEventLevel.DEBUG -> LoggerEventLevel.DEBUG
                RuntimeFFILoggerEventLevel.ERROR -> LoggerEventLevel.ERROR
    }
}

internal fun RuntimeFFIVariableModel.toDomain(): NativeVariableModel {
    return NativeVariableModel(
        key = key,
        value = value,
        type = variableType,
    )
}

internal fun RuntimeFFIBlockModel.toDomain(): NativeBlockModel {
    return NativeBlockModel(
        id = id,
        parentId = parentId,
        parentKey = parentKey,
        version = version,
        slot = slot,
        keyType = keyType,
        key = key,
        scope = scope,
        visibility = visibility,
        position = position,
        data = data.mapValues { (_, data) ->
            NativeBlockDataModel(key = data.key, value = data.value, type = data.dataType)
        },
        properties = properties.mapValues { (_, property) ->
            NativeBlockPropertyModel(
                key = property.key,
                valueMobile = property.valueMobile,
                valueTablet = property.valueTablet,
                valueDesktop = property.valueDesktop,
                type = property.propertyType,
            )
        },
        slots = slots.mapValues { (_, slot) ->
            NativeBlockSlotModel(slot = slot.slot, scope = slot.scope)
        },
        subBlocks = subKeys,
    )
}

internal fun RuntimeFFIActionModel.toDomain(): NativeActionModel {
    return NativeActionModel(
        id = id,
        key = key,
        event = event,
        triggers = triggers.map { it.toDomain() },
    )
}

private fun RuntimeFFIActionTriggerModel.toDomain(): NativeActionTriggerModel {
    return NativeActionTriggerModel(
        name = name,
        id = id,
        parentId = parentId,
        version = version,
        keyType = keyType,
        event = event,
        scope = scope,
        properties = properties.mapValues { (_, property) ->
            NativeActionTriggerPropertyModel(
                key = property.key,
                value = property.value,
                type = property.propertyType,
            )
        },
        data = data.mapValues { (_, data) ->
            NativeActionTriggerDataModel(key = data.key, value = data.value, type = data.dataType)
        },
        events = events.mapValues { (_, event) ->
            NativeActionTriggerEventModel(event = event.event, scope = event.scope)
        },
        subTriggers = emptyList(),
    )
}

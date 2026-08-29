import SwiftSyntax

public protocol NativeMeta: Encodable {}

public enum NativeKind {
    case action
    case block
}

public struct DataMeta: NativeMeta {
    public var position: Int
    public var key: String
    public var type: String
    public var description: String
    public var deprecated: Bool
    public var deprecatedReason: String
    public var block: AttributeSyntax?
    public var variable: PatternBindingSyntax?
    public var value: String

    init(
        position: Int,
        key: String,
        type: String,
        description: String,
        deprecated: Bool,
        deprecatedReason: String,
        block: AttributeSyntax? = nil,
        variable: PatternBindingSyntax? = nil,
        value: String
    ) {
        self.position = position
        self.key = key
        self.type = type
        self.description = description
        self.block = block
        self.variable = variable
        self.deprecated = deprecated
        self.deprecatedReason = deprecatedReason
        self.value = value
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(self.key, forKey: .key)
        try container.encode(TypeUtils.typeMapToJson(self.type), forKey: .type)
        try container.encode(self.description, forKey: .description)
        try container.encode(self.deprecated, forKey: .deprecated)
        try container.encode(self.deprecatedReason, forKey: .deprecatedReason)
    }

    private enum CodingKeys: String, CodingKey {
        case key, type, description, deprecated, deprecatedReason
    }
}

public struct BindingDataMeta: NativeMeta {
    public let data: DataMeta

    init(data: DataMeta) {
        self.data = data
    }

    public func encode(to encoder: Encoder) throws {
        try data.encode(to: encoder)
    }
}

public struct ValuePickerOption: Encodable {
    public var id: String
    public var text: String
}

public struct PropertyMeta: NativeMeta {
    public var position: Int
    public var key: String
    public var value: String
    public var type: String
    public var description: String
    public var deprecated: Bool
    public var deprecatedReason: String
    public var valuePicker: String
    public var valuePickerOptions: [ValuePickerOption]
    public var valuePickerGroup: String
    public var block: AttributeSyntax?
    public var variable: PatternBindingSyntax?

    init(
        position: Int,
        key: String,
        value: String,
        type: String,
        description: String,
        deprecated: Bool,
        deprecatedReason: String,
        valuePicker: String,
        valuePickerOptions: [ValuePickerOption],
        valuePickerGroup: String,
        block: AttributeSyntax? = nil,
        variable: PatternBindingSyntax? = nil
    ) {
        self.position = position
        self.key = key
        self.value = value
        self.type = type
        self.description = description
        self.valuePicker = valuePicker
        self.valuePickerOptions = valuePickerOptions
        self.valuePickerGroup = valuePickerGroup
        self.block = block
        self.variable = variable
        self.deprecated = deprecated
        self.deprecatedReason = deprecatedReason
    }

    private enum CodingKeys: String, CodingKey {
        case key, type, description, deprecated, deprecatedReason, value, valuePicker, valuePickerGroup, valuePickerOptions
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(self.key, forKey: .key)
        try container.encode(TypeUtils.typeMapToJson(self.type), forKey: .type)
        try container.encode(self.description, forKey: .description)
        try container.encode(self.value, forKey: .value)
        try container.encode(TypeUtils.valuePickerMapJson(self.valuePicker), forKey: .valuePicker)
        try container.encode(self.valuePickerGroup, forKey: .valuePickerGroup)
        try container.encode(TypeUtils.valuePickerOptionsMapToJson(self.valuePickerOptions), forKey: .valuePickerOptions)
        try container.encode(self.deprecated, forKey: .deprecated)
        try container.encode(self.deprecatedReason, forKey: .deprecatedReason)
    }
}

public struct EventMeta: NativeMeta {
    public var kind: NativeKind
    public var position: Int
    public var event: String
    public var scope: String?
    public var description: String
    public var deprecated: Bool
    public var deprecatedReason: String
    public var dataBindings: [String] = []
    public var isOptionalFunction: Bool
    public var block: AttributeSyntax?
    public var variable: PatternBindingSyntax?

    init(
        kind: NativeKind,
        position: Int,
        event: String,
        scope: String?,
        description: String,
        deprecated: Bool,
        deprecatedReason: String,
        dataBindings: [String],
        isOptionalFunction: Bool,
        block: AttributeSyntax? = nil,
        variable: PatternBindingSyntax? = nil
    ) {
        self.kind = kind
        self.position = position
        self.event = event
        self.scope = scope
        self.description = description
        self.dataBindings = dataBindings
        self.isOptionalFunction = isOptionalFunction
        self.block = block
        self.variable = variable
        self.deprecated = deprecated
        self.deprecatedReason = deprecatedReason
    }

    private enum CodingKeys: String, CodingKey {
        case event, scope, description, deprecated, deprecatedReason, dataBindings
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(self.event, forKey: .event)
        try container.encodeIfPresent(self.scope, forKey: .scope)
        try container.encode(self.description, forKey: .description)
        try container.encode(self.deprecated, forKey: .deprecated)
        try container.encode(self.deprecatedReason, forKey: .deprecatedReason)
    }
}

public struct SlotMeta: NativeMeta {
    public var position: Int
    public var slot: String
    public var scope: String?
    public var description: String
    public var deprecated: Bool
    public var deprecatedReason: String
    public var dataBindings: [String] = []
    public var hasBlockIndex: Bool
    public var hasBlockScope: Bool
    public var isOptionalFunction: Bool
    public var block: AttributeSyntax?
    public var variable: PatternBindingSyntax?

    init(
        position: Int,
        slot: String,
        scope: String?,
        description: String,
        deprecated: Bool,
        deprecatedReason: String,
        dataBindings: [String],
        hasBlockIndex: Bool,
        hasBlockScope: Bool,
        isOptionalFunction: Bool,
        block: AttributeSyntax? = nil,
        variable: PatternBindingSyntax? = nil
    ) {
        self.position = position
        self.slot = slot
        self.scope = scope
        self.description = description
        self.dataBindings = dataBindings
        self.hasBlockIndex = hasBlockIndex
        self.hasBlockScope = hasBlockScope
        self.isOptionalFunction = isOptionalFunction
        self.block = block
        self.variable = variable
        self.deprecated = deprecated
        self.deprecatedReason = deprecatedReason
    }

    private enum CodingKeys: String, CodingKey {
        case slot, scope, description, deprecated, deprecatedReason, dataBindings
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        try container.encode(self.slot, forKey: .slot)
        try container.encodeIfPresent(self.scope, forKey: .scope)
        try container.encode(self.description, forKey: .description)
        try container.encode(self.deprecated, forKey: .deprecated)
        try container.encode(self.deprecatedReason, forKey: .deprecatedReason)
        if !self.dataBindings.isEmpty {
            try container.encode(self.dataBindings, forKey: .dataBindings)
        }
    }
}

public struct ActionMeta: NativeMeta {
    public var parameterClass: String
    public var functionName: String
    public var functionParamName: String
    public var isAsync: Bool

    init(parameterClass: String, functionName: String, functionParamName: String, isAsync: Bool) {
        self.parameterClass = parameterClass
        self.functionName = functionName
        self.functionParamName = functionParamName
        self.isAsync = isAsync
    }
}

public struct ExtraParamMeta: NativeMeta {
    public var position: Int
    public var key: String
    public var type: String
    public var variable: PatternBindingSyntax?

    init(position: Int, key: String, type: String, variable: PatternBindingSyntax? = nil) {
        self.position = position
        self.key = key
        self.type = type
        self.variable = variable
    }

    private enum CodingKeys: String, CodingKey {
        case key, type
    }
}

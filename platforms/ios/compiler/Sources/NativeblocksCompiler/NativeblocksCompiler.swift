public typealias BlockIndex = Int

@attached(peer, names: suffixed(Block))
public macro NativeBlock(
    name: String,
    keyType: String,
    description: String,
    scope: String = "",
    version: Int = 1,
    versionName: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockMacro")

@attached(peer)
public macro NativeBlockData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockDataMacro")

@attached(peer)
@available(*, deprecated, message: "Properties are being replaced by data; declare block arguments with @NativeBlockData.")
public macro NativeBlockProp(
    description: String = "",
    valuePicker: NativeBlockValuePicker = NativeBlockValuePicker.TEXT_INPUT,
    valuePickerOptions: [NativeBlockValuePickerOption] = [],
    valuePickerGroup: NativeBlockValuePickerPosition = NativeBlockValuePickerPosition("General"),
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockPropMacro")

@attached(peer)
public macro NativeBlockEvent(
    description: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockEventMacro")

@attached(peer)
public macro NativeBlockSlot(
    description: String = "",
    scope: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockSlotMacro")

public enum NativeBlockValuePicker {
    case TEXT_INPUT
    case TEXT_AREA_INPUT
    case NUMBER_INPUT
    case DROPDOWN
    case COMBOBOX_INPUT
    case COLOR_PICKER
}

public struct NativeBlockValuePickerOption {
    var id: String
    var text: String
    public init(_ id: String, _ text: String) {
        self.id = id
        self.text = text
    }
}

public struct NativeBlockValuePickerPosition {
    var text: String
    public init(_ text: String) {
        self.text = text
    }
}

@attached(peer, names: suffixed(Action))
public macro NativeAction(
    name: String,
    keyType: String,
    description: String,
    scope: String = "",
    version: Int = 1,
    versionName: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionMacro")


@attached(peer)
public macro NativeActionParameter(description: String = "") =
    #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionParameterMacro")


@attached(peer)
public macro NativeActionFunction(description: String = "") =
    #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionFunctionMacro")


@attached(peer)
public macro NativeActionData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionDataMacro")


@attached(peer)
@available(*, deprecated, message: "Properties are being replaced by data; declare action arguments with @NativeActionData.")
public macro NativeActionProp(
    description: String = "",
    valuePicker: NativeActionValuePicker = NativeActionValuePicker.TEXT_INPUT,
    valuePickerOptions: [NativeActionValuePickerOption] = [],
    valuePickerGroup: NativeActionValuePickerPosition = NativeActionValuePickerPosition("General"),
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionPropMacro")


@attached(peer)
public macro NativeActionEvent(
    description: String = "",
    scope: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionEventMacro")

public enum NativeActionValuePicker {
    case TEXT_INPUT
    case TEXT_AREA_INPUT
    case NUMBER_INPUT
    case DROPDOWN
    case COMBOBOX_INPUT
    case COLOR_PICKER
    case SCRIPT_AREA_INPUT
}


public struct NativeActionValuePickerOption {
    var id: String
    var text: String
    public init(_ id: String, _ text: String) {
        self.id = id
        self.text = text
    }
}

public struct NativeActionValuePickerPosition {
    var text: String
    public init(_ text: String) {
        self.text = text
    }
}

@attached(peer, names: suffixed(Modifier))
public macro NativeModifier(
    name: String,
    keyType: String,
    description: String,
    scope: String = "",
    version: Int = 1,
    versionName: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeModifierMacro")

@attached(peer)
public macro NativeModifierData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeModifierDataMacro")

@attached(peer)
public macro NativeModifierEvent(
    description: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeModifierEventMacro")


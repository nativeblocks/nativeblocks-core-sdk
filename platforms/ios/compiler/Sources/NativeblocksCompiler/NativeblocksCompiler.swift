public typealias BlockIndex = Int

@attached(peer, names: suffixed(Block))
public macro Block(
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
public macro BlockData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockDataMacro")

@attached(peer)
@available(*, deprecated, message: "Properties are being replaced by data; declare block arguments with @BlockData.")
public macro BlockProp(
    description: String = "",
    valuePicker: BlockValuePicker = BlockValuePicker.TEXT_INPUT,
    valuePickerOptions: [BlockValuePickerOption] = [],
    valuePickerGroup: BlockValuePickerPosition = BlockValuePickerPosition("General"),
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockPropMacro")

@attached(peer)
public macro BlockEvent(
    description: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockEventMacro")

@attached(peer)
public macro BlockSlot(
    description: String = "",
    scope: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeBlockSlotMacro")

public enum BlockValuePicker {
    case TEXT_INPUT
    case TEXT_AREA_INPUT
    case NUMBER_INPUT
    case DROPDOWN
    case COMBOBOX_INPUT
    case COLOR_PICKER
}

public struct BlockValuePickerOption {
    var id: String
    var text: String
    public init(_ id: String, _ text: String) {
        self.id = id
        self.text = text
    }
}

public struct BlockValuePickerPosition {
    var text: String
    public init(_ text: String) {
        self.text = text
    }
}

@attached(peer, names: suffixed(Action))
public macro Action(
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
public macro ActionParameter(description: String = "") =
    #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionParameterMacro")


@attached(peer)
public macro ActionFunction(description: String = "") =
    #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionFunctionMacro")


@attached(peer)
public macro ActionData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionDataMacro")


@attached(peer)
@available(*, deprecated, message: "Properties are being replaced by data; declare action arguments with @ActionData.")
public macro ActionProp(
    description: String = "",
    valuePicker: ActionValuePicker = ActionValuePicker.TEXT_INPUT,
    valuePickerOptions: [ActionValuePickerOption] = [],
    valuePickerGroup: ActionValuePickerPosition = ActionValuePickerPosition("General"),
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionPropMacro")


@attached(peer)
public macro ActionEvent(
    description: String = "",
    scope: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeActionEventMacro")

public enum ActionValuePicker {
    case TEXT_INPUT
    case TEXT_AREA_INPUT
    case NUMBER_INPUT
    case DROPDOWN
    case COMBOBOX_INPUT
    case COLOR_PICKER
    case SCRIPT_AREA_INPUT
}


public struct ActionValuePickerOption {
    var id: String
    var text: String
    public init(_ id: String, _ text: String) {
        self.id = id
        self.text = text
    }
}

public struct ActionValuePickerPosition {
    var text: String
    public init(_ text: String) {
        self.text = text
    }
}

@attached(peer, names: suffixed(Modifier))
public macro Modifier(
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
public macro ModifierData(
    description: String = "",
    deprecated: Bool = false,
    deprecatedReason: String = "",
    defaultValue: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeModifierDataMacro")

@attached(peer)
public macro ModifierEvent(
    description: String = "",
    dataBinding: [String] = [],
    deprecated: Bool = false,
    deprecatedReason: String = ""
) = #externalMacro(module: "NativeblocksCompilerMacros", type: "NativeModifierEventMacro")


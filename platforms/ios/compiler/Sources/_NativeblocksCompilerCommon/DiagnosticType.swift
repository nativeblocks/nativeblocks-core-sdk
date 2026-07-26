import SwiftDiagnostics

public enum DiagnosticType: String, DiagnosticMessage {
    case notAStruct
    case notAClass
    case singleVariableLimit
    case blockSlotParamLimit
    case functionTypeError
    case eventTypeMisMachParamCount
    case primitiveTypeSupported
    case eventDataMissing
    case eventDistinctThen
    case requiredNativeActionFunction
    case requiredNativeActionFunctionParameter
    case actionNotSupportedThrows
    case multiAttributes

    public var severity: DiagnosticSeverity { return .error }

    public var message: String {
        switch self {
        case .notAStruct:
            return "Only structs are supported."
        case .notAClass:
            return "Only classes are supported."
        case .singleVariableLimit:
            return "Wrap a single variable only."
        case .blockSlotParamLimit:
            return "Must have zero param or 'BlockIndex' or 'Any' or both parameter as order."
        case .functionTypeError:
            return "Expected a function."
        case .eventTypeMisMachParamCount:
            return "Parameter count must match 'dataBinding'."
        case .primitiveTypeSupported:
            return "Only primitive types are supported."
        case .eventDataMissing:
            return "Add '@NativeBlockData' with matching type and name."
        case .requiredNativeActionFunction:
            return "Add one '@NativeActionFunction'."
        case .requiredNativeActionFunctionParameter:
            return "'@NativeActionFunction' needs a struct with '@NativeActionParameter'."
        case .eventDistinctThen:
            return "'then' in '@NativeActionEvent' must be unique."
        case .multiAttributes:
            return "Multi attribute are not supported."
        case .actionNotSupportedThrows:
            return
                "'@NativeActionFunction' doesn't support throws. Use '@NativeActionEvent(then: Then.FAILURE)'."
        }
    }

    public var diagnosticID: MessageID {
        MessageID(domain: "NativeblocksCompilerMacros", id: rawValue)
    }
}

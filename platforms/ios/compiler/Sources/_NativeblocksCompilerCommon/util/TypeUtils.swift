import Foundation

public enum TypeUtils {
    /// Builds the expression converting a raw string value into the declared Swift type.
    /// Shared by data and properties, for both blocks and actions.
    public static func valueConversion(
        type: String,
        source: String,
        defaultValue: String,
        instance: String
    ) -> String {
        switch type.uppercased() {
        case "STRING":
            return """
                \(source) ?? "\(defaultValue)"
                """
        case "INT", "INT64", "INT32", "INT16", "INT8", "UINT", "UINT64", "UINT32", "UINT16", "UINT8",
            "FLOAT", "FLOAT80", "FLOAT64", "FLOAT32", "FLOAT16", "DOUBLE":
            return """
                \(type)(\(source) ?? "") ?? \(defaultValue.isEmpty ? "0" : defaultValue)
                """
        case "CGFLOAT":
            return """
                (\(source) ?? "").toCGFloat() ?? \(defaultValue.isEmpty ? "0.0" : defaultValue)
                """
        case "BOOL":
            return """
                Bool(\(source) ?? "") ?? \(defaultValue.isEmpty ? "false" : defaultValue)
                """
        default:
            return """
                NativeblocksManager.getInstance(name: \(instance)).getTypeConverter(\(type).self).fromString(\(source) ?? "\(defaultValue)")
                """
        }
    }

    static func typeMapToJson(_ type: String) -> String? {
        switch type.uppercased() {
        case "STRING":
            return "STRING"
        case "INT", "INT32", "INT16", "INT8", "UINT", "UINT32", "UINT16", "UINT8":
            return "INT"
        case "INT64", "UINT64":
            return "LONG"
        case "FLOAT", "FLOAT80", "FLOAT64", "FLOAT32", "FLOAT16":
            return "FLOAT"
        case "DOUBLE", "CGFLOAT":
            return "DOUBLE"
        case "BOOL":
            return "BOOLEAN"
        default:
            return nil
        }
    }

    static func thenMapToJson(_ then: String?) -> String {
        switch then?.uppercased() {
        case "SUCCESS":
            return "SUCCESS"
        case "FAILURE":
            return "FAILURE"
        case "NEXT":
            return "NEXT"
        case "END":
            return "END"
        default:
            return "END"
        }
    }

    static func valuePickerMapJson(_ type: String) -> String? {
        switch type.uppercased() {
        case "TEXT_INPUT":
            return "text-input"
        case "TEXT_AREA_INPUT":
            return "text-area-input"
        case "NUMBER_INPUT":
            return "number-input"
        case "DROPDOWN":
            return "dropdown"
        case "COLOR_PICKER":
            return "color-picker"
        case "COMBOBOX_INPUT":
            return "combobox-input"
        case "SCRIPT_AREA_INPUT":
            return "script-area-input"
        default:
            return nil
        }
    }

    static func valuePickerOptionsMapToJson(_ options: [ValuePickerOption]) throws -> String {
        return try String(data: JSONEncoder().encode(options), encoding: .utf8) ?? "[]"
    }
}

import Foundation

enum TypeUtils {
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

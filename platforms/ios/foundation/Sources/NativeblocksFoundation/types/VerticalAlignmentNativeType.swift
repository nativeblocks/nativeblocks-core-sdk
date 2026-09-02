import NativeblocksRuntime
import SwiftUI

class VerticalAlignmentNativeType: INativeType<VerticalAlignment> {
    let defaultString = "top"
    let dafault = VerticalAlignment.top

    override required init() {
        super.init()
    }

    override func toString(_ input: VerticalAlignment?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .top:
            return "top"
        case .bottom:
            return "bottom"
        case .center:
            return "center"
        case .firstTextBaseline:
            return "firstTextBaseline"
        case .lastTextBaseline:
            return "lastTextBaseline"
        default:
            return defaultString
        }
    }

    override func fromString(_ input: String?) -> VerticalAlignment {
        guard let input = input else { return dafault }
        switch input {
        case "top": return .top
        case "bottom": return .bottom
        case "center": return .center
        case "firstTextBaseline": return .firstTextBaseline
        case "lastTextBaseline": return .lastTextBaseline
        default: return dafault
        }
    }
}

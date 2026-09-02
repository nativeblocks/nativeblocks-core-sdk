import NativeblocksRuntime
import SwiftUI

class FontDesignNativeType: INativeType<Font.Design> {
    let defaultString = "default"
    let dafault = Font.Design.default

    override required init() {
        super.init()
    }

    override func toString(_ input: Font.Design?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .default:
            return "default"
        case .monospaced:
            return "monospaced"
        case .rounded:
            return "rounded"
        case .serif:
            return "serif"
        default:
            return defaultString
        }
    }

    override func fromString(_ input: String?) -> Font.Design {
        guard let input = input else { return dafault }
        switch input {
        case "default": return .default
        case "monospaced": return .monospaced
        case "rounded": return .rounded
        case "serif": return .serif
        default: return dafault
        }
    }
}

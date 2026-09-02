import NativeblocksRuntime
import SwiftUI

class TextAlignmentNativeType: INativeType<TextAlignment> {
    let defaultString = "leading"
    let dafault: TextAlignment = .leading

    override required init() {
        super.init()
    }

    override func toString(_ input: TextAlignment?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .leading:
            return "leading"
        case .center:
            return "center"
        case .trailing:
            return "trailing"
        }
    }

    override func fromString(_ input: String?) -> TextAlignment {
        guard let input = input else { return dafault }
        switch input {
        case "leading":
            return .leading
        case "center":
            return .center
        case "trailing":
            return .trailing
        default:
            return dafault
        }
    }
}

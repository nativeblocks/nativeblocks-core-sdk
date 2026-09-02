import NativeblocksRuntime
import SwiftUI

class LayoutDirectionNativeType: INativeType<LayoutDirection> {
    let defaultString = "LTR"
    let dafault = LayoutDirection.leftToRight

    override required init() {
        super.init()
    }

    override func toString(_ input: LayoutDirection?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .leftToRight:
            return "LTR"
        case .rightToLeft:
            return "RTL"
        default:
            return defaultString
        }
    }

    override func fromString(_ input: String?) -> LayoutDirection {
        guard let input = input else { return dafault }
        switch input.uppercased() {
        case "RTL":
            return .rightToLeft
        case "LTR":
            return .leftToRight
        default:
            return dafault
        }
    }
}

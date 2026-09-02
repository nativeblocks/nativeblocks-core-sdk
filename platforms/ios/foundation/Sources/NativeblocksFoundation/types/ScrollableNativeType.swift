import NativeblocksRuntime
import SwiftUI

class ScrollableNativeType: INativeType<Axis.Set> {
    let defaultString = "both"
    let dafault: Axis.Set = [.vertical, .horizontal]

    override required init() {
        super.init()
    }

    override func toString(_ input: Axis.Set?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .horizontal:
            return "horizontal"
        case .vertical:
            return "vertical"
        case [.vertical, .horizontal]:
            return "both"
        default:
            return defaultString
        }
    }

    override func fromString(_ input: String?) -> Axis.Set {
        guard let input = input else { return dafault }
        switch input {
        case "horizontal": return .horizontal
        case "vertical": return .vertical
        case "both": return [.vertical, .horizontal]
        default: return dafault
        }
    }
}

import NativeblocksRuntime
import SwiftUI

class FontWeightNativeType: INativeType<Font.Weight> {
    let defaultString = "regular"
    let dafault = Font.Weight.regular

    override required init() {
        super.init()
    }

    override func toString(_ input: Font.Weight?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .ultraLight:
            return "ultraLight"
        case .thin:
            return "thin"
        case .light:
            return "light"
        case .regular:
            return "regular"
        case .medium:
            return "medium"
        case .semibold:
            return "semibold"
        case .bold:
            return "bold"
        case .heavy:
            return "heavy"
        case .black:
            return "black"
        default:
            return defaultString
        }
    }

    override func fromString(_ input: String?) -> Font.Weight {
        guard let input = input else { return dafault }
        switch input {
        case "ultraLight": return .ultraLight
        case "thin": return .thin
        case "light": return .light
        case "regular": return .regular
        case "medium": return .medium
        case "semibold": return .semibold
        case "bold": return .bold
        case "heavy": return .heavy
        case "black": return .black
        default: return dafault
        }
    }
}

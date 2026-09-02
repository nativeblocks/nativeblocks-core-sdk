import NativeblocksRuntime
import SwiftUI

class HorizontalAlignmentNativeType: INativeType<HorizontalAlignment> {
    let defaultString = "leading"
    let dafault = HorizontalAlignment.leading

    override required init() {
        super.init()
    }

    override func toString(_ input: HorizontalAlignment?) -> String {
        guard let input = input else { return defaultString }
        if #available(iOS 16.0, *) {
            switch input {
            case .leading:
                return "leading"
            case .trailing:
                return "trailing"
            case .center:
                return "center"
            case .listRowSeparatorLeading:
                return "listRowSeparatorLeading"
            case .listRowSeparatorTrailing:
                return "listRowSeparatorTrailing"
            default:
                return defaultString
            }
        } else {
            switch input {
            case .leading:
                return "leading"
            case .trailing:
                return "trailing"
            case .center:
                return "center"
            default:
                return defaultString
            }
        }
    }

    override func fromString(_ input: String?) -> HorizontalAlignment {
        guard let input = input else { return dafault }
        switch input {
        case "leading": return .leading
        case "trailing": return .trailing
        case "listRowSeparatorLeading":
            if #available(iOS 16.0, *) {
                return .listRowSeparatorLeading
            } else {
                return dafault
            }
        case "listRowSeparatorTrailing":
            if #available(iOS 16.0, *) {
                return .listRowSeparatorTrailing
            } else {
                return dafault
            }
        case "center": return .center
        default: return dafault
        }
    }
}

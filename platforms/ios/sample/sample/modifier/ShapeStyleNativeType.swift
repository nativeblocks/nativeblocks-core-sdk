import NativeblocksRuntime
import SwiftUI

/// The outline a shape modifier draws.
public enum ShapeStyleType {
    case rectangle
    case roundedRectangle
    case circle
    case capsule
}

class ShapeStyleNativeType: INativeType<ShapeStyleType> {
    let defaultString = "rectangle"
    let dafault: ShapeStyleType = .rectangle

    nonisolated override required init() {
        super.init()
    }

    nonisolated override func toString(_ input: ShapeStyleType?) -> String {
        guard let input = input else { return defaultString }
        switch input {
        case .rectangle:
            return "rectangle"
        case .roundedRectangle:
            return "roundedRectangle"
        case .circle:
            return "circle"
        case .capsule:
            return "capsule"
        }
    }

    nonisolated override func fromString(_ input: String?) -> ShapeStyleType {
        guard let input = input else { return dafault }
        switch input {
        case "roundedRectangle":
            return .roundedRectangle
        case "circle":
            return .circle
        case "capsule":
            return .capsule
        default:
            return .rectangle
        }
    }
}

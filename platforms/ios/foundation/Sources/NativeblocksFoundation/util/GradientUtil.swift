import Foundation
import SwiftUI

let GRADIENT_LINEAR = "linear"
let GRADIENT_RADIAL = "radial"
let GRADIENT_SWEEP = "sweep"

/// Parses a comma-separated list of hexadecimal colors (e.g. "#004FF0, #00FFFFFF"),
/// dropping entries that fail to parse.
func parseColors(_ colors: String) -> [Color] {
    return colors.split(separator: ",").compactMap { parseColor(String($0)) }
}

func parseColor(_ value: String) -> Color? {
    let hex = value.trimmingCharacters(in: .whitespacesAndNewlines).replacingOccurrences(of: "#", with: "")
    guard hex.count == 6 || hex.count == 8, hex.allSatisfy({ $0.isHexDigit }) else { return nil }

    var rgb: UInt64 = 0
    guard Scanner(string: hex).scanHexInt64(&rgb) else { return nil }

    if hex.count == 6 {
        return Color(
            red: CGFloat((rgb & 0xFF0000) >> 16) / 255.0,
            green: CGFloat((rgb & 0x00FF00) >> 8) / 255.0,
            blue: CGFloat(rgb & 0x0000FF) / 255.0
        )
    }
    return Color(
        red: CGFloat((rgb & 0x00FF_0000) >> 16) / 255.0,
        green: CGFloat((rgb & 0x0000_FF00) >> 8) / 255.0,
        blue: CGFloat(rgb & 0x0000_00FF) / 255.0,
        opacity: CGFloat((rgb & 0xFF00_0000) >> 24) / 255.0
    )
}

/// Builds a gradient style of the given type. Linear gradients honor `angleDegrees`,
/// where 0 flows start-to-end and 90 flows top-to-bottom.
func gradientStyle(_ type: String, _ colors: [Color], _ angleDegrees: Double) -> AnyShapeStyle {
    switch type {
    case GRADIENT_RADIAL:
        return AnyShapeStyle(EllipticalGradient(colors: colors))
    case GRADIENT_SWEEP:
        return AnyShapeStyle(AngularGradient(colors: colors, center: .center))
    default:
        let radians = angleDegrees * .pi / 180
        let dx = cos(radians) / 2
        let dy = sin(radians) / 2
        return AnyShapeStyle(
            LinearGradient(
                colors: colors,
                startPoint: UnitPoint(x: 0.5 - dx, y: 0.5 - dy),
                endPoint: UnitPoint(x: 0.5 + dx, y: 0.5 + dy)
            )
        )
    }
}

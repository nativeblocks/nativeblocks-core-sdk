import NativeblocksRuntime
import SwiftUI

class ColorNativeType: INativeType<Color> {
    let defaultString = "#00000000"
    let dafault = Color.black.opacity(0)

    override required init() {
        super.init()
    }

    override func toString(_ input: Color?) -> String {
        guard let color = input else {
            return defaultString
        }

        #if canImport(UIKit)
            let platformColor = UIColor(color)
        #elseif canImport(AppKit)
            let platformColor = NSColor(color)
        #endif

        var red: CGFloat = 0
        var green: CGFloat = 0
        var blue: CGFloat = 0
        var alpha: CGFloat = 0

        #if canImport(UIKit)
            guard platformColor.getRed(&red, green: &green, blue: &blue, alpha: &alpha) else {
                return "#00000000"
            }
        #elseif canImport(AppKit)
            guard let convertedColor = platformColor.usingColorSpace(.deviceRGB) else {
                return defaultString
            }
            red = convertedColor.redComponent
            green = convertedColor.greenComponent
            blue = convertedColor.blueComponent
            alpha = convertedColor.alphaComponent
        #endif

        return String(
            format: "#%02X%02X%02X%02X",
            Int(round(alpha * 255)),
            Int(round(red * 255)),
            Int(round(green * 255)),
            Int(round(blue * 255))
        )
    }

    override func fromString(_ input: String?) -> Color {
        guard let colorString = input else {
            return dafault
        }

        var hexSanitized = colorString.trimmingCharacters(in: .whitespacesAndNewlines)
        hexSanitized = hexSanitized.replacingOccurrences(of: "#", with: "")

        var rgb: UInt64 = 0
        var r: CGFloat = 0.0
        var g: CGFloat = 0.0
        var b: CGFloat = 0.0
        var a: CGFloat = 1.0

        let length = hexSanitized.count

        guard Scanner(string: hexSanitized).scanHexInt64(&rgb) else { return dafault }

        if length == 6 {
            r = CGFloat((rgb & 0xFF0000) >> 16) / 255.0
            g = CGFloat((rgb & 0x00FF00) >> 8) / 255.0
            b = CGFloat(rgb & 0x0000FF) / 255.0
        } else if length == 8 {
            a = CGFloat((rgb & 0xFF00_0000) >> 24) / 255.0
            r = CGFloat((rgb & 0x00FF_0000) >> 16) / 255.0
            g = CGFloat((rgb & 0x0000_FF00) >> 8) / 255.0
            b = CGFloat(rgb & 0x0000_00FF) / 255.0
        } else {
            return dafault
        }

        return Color(red: r, green: g, blue: b, opacity: a)
    }
}

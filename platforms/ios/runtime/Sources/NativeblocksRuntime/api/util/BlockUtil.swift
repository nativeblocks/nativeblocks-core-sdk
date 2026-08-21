import Foundation
import SwiftUI

/// The device width bucket a frame is being rendered at.
@available(*, deprecated, message: "Breakpoints are being dropped along with properties; declare block arguments with @NativeBlockData.")
public enum NativeDeviceWidth {
    case MOBILE
    case TABLET
    case DESKTOP
}

@available(*, deprecated, message: "Breakpoints are being dropped along with properties; declare block arguments with @NativeBlockData.")
private struct NativeWindowWidthClassKey: EnvironmentKey {
    static let defaultValue: NativeDeviceWidth = .MOBILE
}

extension EnvironmentValues {
    /// The width bucket for the frame currently being rendered. `NativeFrame`
    /// resolves it once and hands it down, so a block reads it a single time and
    /// passes it to `findWindowSizeClass` for every property instead of
    /// re-deriving the size class per property.
    @available(*, deprecated, message: "Breakpoints are being dropped along with properties; declare block arguments with @NativeBlockData.")
    public var nativeWindowWidthClass: NativeDeviceWidth {
        get { self[NativeWindowWidthClassKey.self] }
        set { self[NativeWindowWidthClassKey.self] = newValue }
    }
}

/// Maps SwiftUI's size classes onto the runtime's width buckets.
@available(*, deprecated, message: "Breakpoints are being dropped along with properties; declare block arguments with @NativeBlockData.")
internal func currentWindowWidthClass(
    _ vertical: UserInterfaceSizeClass?,
    _ horizontal: UserInterfaceSizeClass?
) -> NativeDeviceWidth {
    if horizontal == .compact {
        return .MOBILE
    } else if horizontal == .regular, vertical == .regular {
        return .TABLET
    } else {
        return .DESKTOP
    }
}

/// Picks the device-specific value for the given `windowManager`.
///
/// - Parameters:
///   - prop: A `NativeBlockPropertyModel` containing size-related values for different device types.
///   - windowManager: The device width for the current frame.
/// - Returns: The appropriate value, or `nil` when `prop` is nil.
@available(*, deprecated, message: "Breakpoints are being dropped along with properties; declare block arguments with @NativeBlockData.")
public func findWindowSizeClass(_ prop: NativeBlockPropertyModel?, _ windowManager: NativeDeviceWidth) -> String? {
    guard let prop else { return nil }
    switch windowManager {
    case .MOBILE: return prop.valueMobile
    case .TABLET: return prop.valueTablet
    case .DESKTOP: return prop.valueDesktop
    }
}

extension String {
    /// Converts a string representation of a number to a `CGFloat`.
    ///
    /// - Returns: A `CGFloat` if the string can be converted to a number; otherwise, `nil`.
    public func toCGFloat() -> CGFloat? {
        guard let doubleValue = Double(self) else {
            return nil
        }
        return CGFloat(doubleValue)
    }
}

extension Color {
    /// Initializes a `Color` from a hexadecimal string.
    ///
    /// - Parameter hex: A hexadecimal string representing the color. It can be in the format `#RRGGBB` or `#RRGGBBAA`.
    /// - Returns: A `Color` if the hexadecimal string is valid; otherwise, `nil`.
    ///
    /// ### Example Usage:
    /// ```swift
    /// let color = Color(hex: "#FF5733")
    /// print(color) // Output: Optional(Color(red: 1.0, green: 0.34, blue: 0.2, opacity: 1.0))
    /// ```
    public init?(hex: String) {
        var hexSanitized = hex.trimmingCharacters(in: .whitespacesAndNewlines)
        hexSanitized = hexSanitized.replacingOccurrences(of: "#", with: "")

        var rgb: UInt64 = 0

        var r: CGFloat = 0.0
        var g: CGFloat = 0.0
        var b: CGFloat = 0.0
        var a: CGFloat = 1.0

        let length = hexSanitized.count

        guard Scanner(string: hexSanitized).scanHexInt64(&rgb) else { return nil }

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
            return nil
        }

        self.init(red: r, green: g, blue: b, opacity: a)
    }
}

/// Provides the `NativeBlockSlotModel` for the specified slot type if the block supports it.
///
/// - Parameters:
///   - blockContext: The properties of the block, including its sub-blocks.
///   - slotType: The type of slot to check and provide.
/// - Returns: The `NativeBlockSlotModel` for the specified slot type if the block supports it; otherwise, `nil`.
public func blockProvideSlot(blockContext: BlockContext, slotType: String) -> NativeBlockSlotModel? {
    guard blockContext.block.subBlocks?[slotType] != nil else { return nil }
    return blockContext.block.slots[slotType]
}

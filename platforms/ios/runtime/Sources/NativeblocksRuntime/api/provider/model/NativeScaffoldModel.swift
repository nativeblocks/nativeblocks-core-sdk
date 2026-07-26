import Foundation

/// Represents the scaffold model for Nativeblocks, containing metadata about frames and their routes.
public struct NativeScaffoldModel {
    /// A collection of frame routes defined within the scaffold.
    public let frames: [NativeFrameRouteModel]
}

/// Represents a frame route model, which defines a specific UI element and its associated metadata.
public struct NativeFrameRouteModel {
    /// The unique identifier for the frame route.
    public let id: String?

    /// The name of the frame route, used for display or identification.
    public let name: String?

    /// The type of the frame, such as a standard frame, bottom sheet, or dialog.
    public let type: FrameTypeModel?

    /// The route path for this frame, used for navigation.
    public let route: String?

    /// platform of the frame.
    public let platform: String?

    /// A collection of arguments that can be passed to this route.
    public let routeArguments: [NativeRouteArgumentsModel]?
}

/// Represents an argument for a specific route, defined by its name.
public struct NativeRouteArgumentsModel {
    /// The name of the argument.
    public let name: String?
}

/// Defines the types of frames available in Nativeblocks.
public enum FrameTypeModel: String {
    /// A standard frame type, typically a full-screen or main view.
    case frame = "FRAME"

    /// A bottom sheet type, often used for partial overlays on the screen.
    case bottomSheet = "BOTTOM_SHEET"

    /// A dialog type, used for modal interactions.
    case dialog = "DIALOG"

    /// Converts a string into a `FrameTypeModel`. Defaults to `.frame` if the string does not match a known type.
    /// - Parameter type: The string representation of the frame type.
    /// - Returns: A corresponding `FrameTypeModel` value.
    static func fromString(_ type: String) -> FrameTypeModel {
        return FrameTypeModel(rawValue: type) ?? .frame
    }
}

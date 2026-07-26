import Foundation

/// Represents a model for a variable used within the native application.
/// The `NativeVariableModel` is used to define key-value pairs with an associated type.
public struct NativeVariableModel: Hashable, Codable {
    /// The key associated with the variable.
    public var key: String

    /// The value of the variable.
    public var value: String

    /// The type of the variable, indicating the nature of the value.
    public var type: String

    /// Equality operator to compare two `NativeVariableModel` instances.
    /// - Parameters:
    ///   - lhs: The left-hand side `NativeVariableModel`.
    ///   - rhs: The right-hand side `NativeVariableModel`.
    /// - Returns: `true` if both instances are equal, `false` otherwise.
    public static func == (lhs: NativeVariableModel, rhs: NativeVariableModel) -> Bool {
        return lhs.key == rhs.key && lhs.value == rhs.value && lhs.type == rhs.type
    }
}

extension NativeVariableModel {
    public func copy(
        key: String? = nil,
        value: String? = nil,
        type: String? = nil
    ) -> NativeVariableModel {
        return NativeVariableModel(
            key: key ?? self.key,
            value: value ?? self.value,
            type: type ?? self.type
        )
    }
}

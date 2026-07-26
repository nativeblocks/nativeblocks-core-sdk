import Foundation

/// A base class for implementing type converters for Native types.
/// Subclasses must override `toString` and `fromString` to provide conversion logic.
///
/// - Type Parameter T: The type being converted.
open class INativeType<T> {
    ///
    /// Initializes a new instance of the type converter.
    ///
    public init() {}

    ///
    /// Converts a value of type `T` to a string representation.
    ///
    /// - Parameter input: The value to convert.
    /// - Returns: A string representation of the value.
    /// - Precondition: This method must be overridden by subclasses.
    ///
    open func toString(_ input: T?) -> String {
        fatalError("toString must be overridden")
    }

    ///
    /// Converts a string representation back to a value of type `T`.
    ///
    /// - Parameter input: The string to convert.
    /// - Returns: A value of type `T`.
    /// - Precondition: This method must be overridden by subclasses.
    ///
    open func fromString(_ input: String?) -> T {
        fatalError("fromString must be overridden")
    }
}

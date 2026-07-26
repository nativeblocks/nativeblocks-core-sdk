import Foundation

/// Protocol that defines the core behavior for managing Wandkit operations.
/// The `Wandkit` protocol provides methods to set up and destroy the Wandkit instance, making it suitable for managing the lifecycle of Nativeblocks components.
public protocol Wandkit {
    /// Sets up the Wandkit instance with the specified edition.
    /// - Parameter edition: The `NativeblocksEdition` that determines the configuration type, such as cloud or community.
    func setup(edition: NativeblocksEdition, instanceName: String)

    /// Destroys the Wandkit instance and releases all associated resources.
    func destroy()
}

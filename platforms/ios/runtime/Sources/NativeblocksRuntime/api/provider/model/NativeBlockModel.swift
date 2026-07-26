import Foundation

/// Represents a block within a native UI framework, including its properties,
/// data, visibility, position, and hierarchical structure.
public struct NativeBlockModel: Hashable, Codable {
    /// The unique identifier of the block.
    public let id: String

    /// The identifier of the parent block, to establish hierarchy.
    public let parentId: String

    /// The key of the parent block, to establish hierarchy.
    public let parentKey: String

    /// The version of the block.
    public let version: Int

    /// Slot name where the block resides.
    public let slot: String

    /// The type of the key used for this block.
    public let keyType: String

    /// The key associated with this block.
    public let key: String

    /// Visibility of the block (e.g., visible, hidden).
    public let visibility: String

    /// The position of the block, used for ordering within its container.
    public let position: Int

    /// A dictionary of properties associated with this block.
    public let properties: [String: NativeBlockPropertyModel]

    /// A dictionary of data models that belong to this block.
    public let data: [String: NativeBlockDataModel]

    /// A dictionary of slots that represent different areas where sub-blocks can be placed.
    public let slots: [String: NativeBlockSlotModel]

    /// Keys of the sub-blocks nested within this block, grouped by slot name.
    public let subBlocks: [String: [String]]?

    /// Equality operator to compare two `NativeBlockModel` instances.
    public static func == (lhs: NativeBlockModel, rhs: NativeBlockModel) -> Bool {
        return lhs.id == rhs.id && lhs.parentId == rhs.parentId && lhs.keyType == rhs.keyType
            && lhs.key == rhs.key && lhs.visibility == rhs.visibility && lhs.position == rhs.position
            && lhs.properties == rhs.properties && lhs.data == rhs.data && lhs.slots == rhs.slots
            && lhs.subBlocks == rhs.subBlocks
    }
}

/// Represents a property of a native block, including device-specific values and its type.
public struct NativeBlockPropertyModel: Hashable, Codable {
    /// The key associated with the property.
    public let key: String

    /// The value for mobile devices.
    public let valueMobile: String

    /// The value for tablet devices.
    public let valueTablet: String

    /// The value for desktop devices.
    public let valueDesktop: String

    /// The type of the property.
    public let type: String

    /// Equality operator to compare two `NativeBlockPropertyModel` instances.
    public static func == (lhs: NativeBlockPropertyModel, rhs: NativeBlockPropertyModel) -> Bool {
        return lhs.key == rhs.key && lhs.valueMobile == rhs.valueMobile
            && lhs.valueTablet == rhs.valueTablet && lhs.valueDesktop == rhs.valueDesktop
            && lhs.type == rhs.type
    }
}

/// Represents data associated with a native block.
public struct NativeBlockDataModel: Hashable, Codable {
    /// The key for the data entry.
    public let key: String

    /// The value associated with the key.
    public let value: String

    /// The type of the data.
    public let type: String

    /// Equality operator to compare two `NativeBlockDataModel` instances.
    public static func == (lhs: NativeBlockDataModel, rhs: NativeBlockDataModel) -> Bool {
        return lhs.key == rhs.key && lhs.value == rhs.value && lhs.type == rhs.type
    }
}

/// Represents a slot within a native block for holding additional content.
public struct NativeBlockSlotModel: Hashable, Codable {
    /// The slot identifier.
    public let slot: String

    /// Equality operator to compare two `NativeBlockSlotModel` instances.
    public static func == (lhs: NativeBlockSlotModel, rhs: NativeBlockSlotModel) -> Bool {
        return lhs.slot == rhs.slot
    }
}

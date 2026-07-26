import Foundation

/// Represents an action within Nativeblocks, including its event triggers and associated metadata.
public struct NativeActionModel: Hashable, Codable {
    /// The unique identifier of the action.
    public let id: String

    /// The key associated with the action.
    public let key: String

    /// The event that triggers this action.
    public let event: String

    /// The triggers that define the sequence and conditions for executing the action.
    public let triggers: [NativeActionTriggerModel]

    /// Equality operator to compare two `NativeActionModel` instances.
    public static func == (lhs: NativeActionModel, rhs: NativeActionModel) -> Bool {
        return lhs.id == rhs.id && lhs.key == rhs.key && lhs.event == rhs.event
            && lhs.triggers == rhs.triggers
    }
}

/// Represents a trigger for a native action, defining the conditions and outcomes of an action.
public struct NativeActionTriggerModel: Hashable, Codable {
    /// The unique identifier of the trigger.
    public let id: String

    /// The identifier of the parent trigger, to establish hierarchy.
    public let parentId: String

    /// The version of the trigger.
    public let version: Int

    /// The type of key used for the trigger.
    public let keyType: String

    /// The name of the trigger.
    public let name: String

    /// What happens after the trigger is executed.
    public let then: NativeActionTriggerThen

    /// A dictionary of properties associated with this trigger.
    public let properties: [String: NativeActionTriggerPropertyModel]

    /// A dictionary of data models that belong to this trigger.
    public let data: [String: NativeActionTriggerDataModel]

    /// The sub-triggers nested within this trigger.
    public var subTriggers: [NativeActionTriggerModel]? = nil

    /// Equality operator to compare two `NativeActionTriggerModel` instances.
    public static func == (lhs: NativeActionTriggerModel, rhs: NativeActionTriggerModel) -> Bool {
        return lhs.id == rhs.id && lhs.parentId == rhs.parentId && lhs.keyType == rhs.keyType
            && lhs.then == rhs.then && lhs.properties == rhs.properties && lhs.data == rhs.data
    }
}

/// Represents a property model for a trigger, including key-value pairs and their types.
public struct NativeActionTriggerPropertyModel: Hashable, Codable {
    /// The key associated with the property.
    public let key: String

    /// The value of the property.
    public let value: String

    /// The type of the property.
    public let type: String

    /// Equality operator to compare two `NativeActionTriggerPropertyModel` instances.
    public static func == (lhs: NativeActionTriggerPropertyModel, rhs: NativeActionTriggerPropertyModel) -> Bool {
        return lhs.key == rhs.key && lhs.value == rhs.value && lhs.type == rhs.type
    }
}

/// Represents data associated with a trigger, including key-value pairs and their types.
public struct NativeActionTriggerDataModel: Hashable, Codable {
    /// The key associated with the data.
    public let key: String

    /// The value of the data.
    public let value: String

    /// The type of the data.
    public let type: String

    /// Equality operator to compare two `NativeActionTriggerDataModel` instances.
    public static func == (lhs: NativeActionTriggerDataModel, rhs: NativeActionTriggerDataModel) -> Bool {
        return lhs.key == rhs.key && lhs.value == rhs.value && lhs.type == rhs.type
    }
}

/// Represents the possible outcomes for a trigger action.
public enum NativeActionTriggerThen: String, Hashable, Codable {
    /// The action will be marked as successful.
    case success = "SUCCESS"

    /// The action has failed.
    case failure = "FAILURE"

    /// Move to the next step in the action sequence.
    case next = "NEXT"

    /// End the action sequence.
    case end = "END"

    /// Converts a string to a `NativeActionTriggerThen` value, defaulting to `.end` if invalid.
    static func fromThen(_ then: String) -> NativeActionTriggerThen {
        return NativeActionTriggerThen(rawValue: then.uppercased()) ?? .end
    }
}

import Foundation
import SwiftUI

/// Protocol that defines the behavior for a native action.
/// The `INativeAction` protocol is used for implementing custom actions that can handle specific properties and operations.
public protocol INativeAction {
    /// Handles the action with the given properties.
    /// - Parameter actionContext: The properties and state information needed to execute the action.
    func handle(actionContext: ActionContext)
}

/// Defines a contract for view-backed action contractors.
/// Some actions only work while a view of theirs is mounted (a picker, a
/// sheet host, a camera session); a contractor is that view, and every
/// `NativeblocksFrame` keeps the registered ones alive for the frame's lifetime.
public protocol INativeActionContractor {
    /// The view this action contractor needs mounted.
    func actionContractor() -> any View
}

/// Represents the properties associated with a native action.
/// The `ActionContext` struct is used to pass all the necessary information required to perform an action, including variables, blocks, triggers, and callbacks.
public struct ActionContext {
    /// Instance name of NativeblocksManager.
    public let instanceName: String

    /// The index of the item in the list that the action applies to (if applicable).
    public let listItemIndex: Int

    /// A function for retrieving a [NativeVariableModel] by its key
    public let onFindVariable: (String) -> NativeVariableModel?

    /// Callback function to handle changes to a variable.
    public let onUpdateVariable: (NativeVariableModel?) -> Void

    /// A function for retrieving a [NativeBlockModel] by its key
    public let onFindBlock: (String) -> NativeBlockModel?

    /// Callback function to update one property of a block.
    /// - Parameters:
    ///   - blockKey: The key of the block to update.
    ///   - propertyKey: The key of the property to update.
    ///   - valueMobile: The new value for mobile devices.
    ///   - valueTablet: The new value for tablets.
    ///   - valueDesktop: The new value for desktop devices.
    public let onUpdateBlockProperties: (String, String, String, String, String) -> Void

    /// The trigger that is associated with this action.
    public let trigger: NativeActionTriggerModel?

    /// Callback function to handle the next trigger in the sequence.
    public let onHandleNextTrigger: (NativeActionTriggerModel) -> Void

    /// Callback function to handle the success of the current trigger and move to the next trigger.
    public let onHandleSuccessNextTrigger: (NativeActionTriggerModel) -> Void

    /// Callback function to handle the failure of the current trigger and move to the next trigger.
    public let onHandleFailureNextTrigger: (NativeActionTriggerModel) -> Void
}

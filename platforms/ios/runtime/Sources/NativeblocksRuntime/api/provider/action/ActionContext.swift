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
/// The `ActionContext` struct is used to pass all the necessary information required to perform an action, including variables, triggers, and callbacks.
public struct ActionContext {
    /// Instance name of NativeblocksManager.
    public let instanceName: String

    /// The index of the item in the list that the action applies to (if applicable).
    public let listItemIndex: Int

    /// A function for retrieving a [NativeVariableModel] by its key
    public let onFindVariable: (String) -> NativeVariableModel?

    /// Callback function to handle changes to a variable.
    public let onUpdateVariable: (NativeVariableModel?) -> Void

    /// The trigger that is associated with this action.
    public let trigger: NativeActionTriggerModel?

    /// Runs the triggers filed under one of this action's events.
    public let onHandleEvent: (String) -> Void

    /// resolveTemplate Fills in whatever that scope reports, leaving the value alone outside one.
    public let resolveTemplate: (String?) -> String?
}

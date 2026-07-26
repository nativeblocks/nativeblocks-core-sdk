import Foundation
import SwiftUI

/// A constant representing an invalid or non-existent index.
public let NONE_INDEX = -1

/// Represents the properties associated with a native block.
/// The `BlockProps` struct is used to pass all the necessary information required to render a block, including variables, actions, and callbacks.
public struct BlockProps {
    /// Instance name of NativeblocksManager.
    public let instanceName: String

    /// The index of the item in the list that the block applies to (if applicable).
    public let listItemIndex: Int

    /// A function for retrieving a [NativeVariableModel] by its key.
    public let onFindVariable: (String) -> NativeVariableModel?

    /// Callback function to handle changes to a variable.
    public let onVariableChange: (NativeVariableModel) -> Void

    /// A function for retrieving a [NativeActionModel] by its event type.
    public let onFindAction: (String) -> NativeActionModel?

    /// Callback function to handle an action.
    /// - Parameters:
    ///   - listItemIndex: The index of the item in the list that the action applies to.
    ///   - action: The action model to be executed.
    ///   - type: The type of action being performed.
    public let onHandleAction: (Int, NativeActionModel?, String) -> Void

    /// The block model representing the current block.
    public let block: NativeBlockModel

    /// Callback function to render a slot's sub-blocks.
    /// - Parameters:
    ///   - blockKeys: Child block keys grouped by slot name.
    ///   - subSlot: The slot whose children should be rendered.
    ///   - itemIndex: The index of the item in the list for which sub-blocks are rendered.
    ///   - scope: Optional layout scope forwarded by the parent block.
    /// - Returns: An `AnyView` containing the rendered sub-blocks.
    public let onSubBlock: ([String: [String]], NativeBlockSlotModel, Int, Any?) -> AnyView
}

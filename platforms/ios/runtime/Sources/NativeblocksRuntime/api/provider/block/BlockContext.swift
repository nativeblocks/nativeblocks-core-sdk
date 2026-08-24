import Foundation
import SwiftUI

/// A constant representing an invalid or non-existent index.
public let NONE_INDEX = -1

/// Everything a block needs while it renders, handed to it by the tree.
///
/// - Parameters:
///   - instanceName: Instance name of NativeblocksManager.
///   - listItemIndex: Index of the list item this block belongs to, or `NONE_INDEX` outside a list.
///   - onFindVisibility: Resolves the block's visibility value.
///   - onFindVariable: Resolves the value a block data entry points at.
///   - onUpdateVariable: Writes a value back to the variable a block data entry points at.
///   - onFindAction: Finds the action bound to the given event type.
///   - onHandleAction: Runs an action for the given list item index and event type.
///   - block: The block being rendered.
///   - modifier: Modifiers attached to the block, already ordered and scope checked.
///   - onSubBlock: Renders the child blocks of a slot, handing them the index and the slot's scope.
public struct BlockContext {
    public let instanceName: String
    public let listItemIndex: Int
    public let onFindVisibility: () -> String?
    public let onFindVariable: (NativeBlockDataModel?) -> String?
    public let onUpdateVariable: (NativeBlockDataModel?, String) -> Void
    public let onFindAction: (String) -> NativeActionModel?
    public let onHandleAction: (Int, NativeActionModel?, String) -> Void
    public let block: NativeBlockModel
    public let modifier: NativeblocksModifier
    public let onSubBlock: ([String: [String]], NativeBlockSlotModel, Int, Any?) -> AnyView
}

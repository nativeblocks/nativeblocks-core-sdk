import Foundation
import SwiftUI

/// Everything a modifier needs while it decorates the block it is attached to.
///
/// - Parameters:
///   - instanceName: Instance name of NativeblocksManager.
///   - listItemIndex: Index of the list item the host block belongs to, or `NONE_INDEX` outside a list.
///   - onFindVariable: Resolves the value a modifier data entry points at.
///   - onUpdateVariable: Writes a value back to the variable a modifier data entry points at.
///   - onFindAction: Finds the action bound to the given event type.
///   - onHandleAction: Runs an action for the given list item index and event type.
///   - modifier: The modifier being applied.
///   - scope: Layout scope of the slot the host block sits in, or nil when it declares none.
///   - resolveTemplate Fills in whatever that scope reports, leaving the value alone outside one.
public struct ModifierContext {
    public let instanceName: String
    public let listItemIndex: Int
    public let onFindVariable: (NativeBlockModifierDataModel?) -> String?
    public let onUpdateVariable: (NativeBlockModifierDataModel?, String) -> Void
    public let onFindAction: (String) -> NativeActionModel?
    public let onHandleAction: (Int, NativeActionModel?, String) -> Void
    public let modifier: NativeBlockModifierModel
    public let scope: Any?
    public let resolveTemplate: (String?) -> String?
}

/// The modifiers attached to a block, already ordered and scope checked, folded into one value a
/// block applies with `.modifier(blockContext.modifier)`.
public struct NativeblocksModifier: ViewModifier {
    private let apply: (AnyView) -> AnyView

    public init(apply: @escaping (AnyView) -> AnyView) {
        self.apply = apply
    }

    public func body(content: Content) -> some View {
        return apply(AnyView(content))
    }

    /// A chain that changes nothing, for blocks reading from a context that may be nil.
    public static let none = NativeblocksModifier { content in content }
}

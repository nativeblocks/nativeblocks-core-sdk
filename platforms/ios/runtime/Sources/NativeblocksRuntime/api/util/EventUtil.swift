import Foundation

/// Provides an event handler for a specific event type if the block supports it.
///
/// - Parameters:
///   - blockContext: The properties of the block, including its event handling logic.
///   - eventType: The type of event to check and provide a handler for.
/// - Returns: A closure representing the event handler if the event type is supported; otherwise, `nil`.
public func blockProvideEvent(blockContext: BlockContext, eventType: String) -> (() -> Void)? {
    if let action = blockContext.onFindAction(eventType) {
        return { blockHandleEvent(blockContext: blockContext, action: action, eventType: eventType) }
    } else {
        return nil
    }
}

/// Handles an event for a block by invoking the appropriate action handler.
///
/// - Parameters:
///   - blockContext: The properties of the block, including its event handling logic.
///   - action: The action associated with the block for this event.
///   - eventType: The type of event to handle.
private func blockHandleEvent(blockContext: BlockContext, action: NativeActionModel, eventType: String) {
    blockContext.onHandleAction(blockContext.listItemIndex, action, eventType)
}

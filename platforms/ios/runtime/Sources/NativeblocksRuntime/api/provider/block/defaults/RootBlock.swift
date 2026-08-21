import Foundation
import SwiftUI

internal struct RootBlock: View {
    var blockContext: BlockContext
    @Environment(\.nativeWindowWidthClass) var windowManager

    var body: some View {
        let slots = blockContext.block.slots
        let contentSlot = slots["content"]

        VStack {
            if let contentSlot {
                blockContext.onSubBlock(blockContext.block.subBlocks ?? [:], contentSlot, NONE_INDEX, nil)
            }
        }
    }
}

import Foundation
import SwiftUI

internal struct RootBlock: View {
    var blockProps: BlockProps
    @Environment(\.nativeWindowWidthClass) var windowManager

    var body: some View {
        let slots = blockProps.block.slots
        let contentSlot = slots["content"]

        VStack {
            if let contentSlot {
                blockProps.onSubBlock(blockProps.block.subBlocks ?? [:], contentSlot, NONE_INDEX, nil)
            }
        }
    }
}

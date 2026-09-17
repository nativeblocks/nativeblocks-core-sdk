import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "ZStack",
    keyType: "nativeblocks/zstack",
    description: "Stacking layout container; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct ZStack<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Children alignment (topLeading, top, topTrailing, leading, center, trailing, bottomLeading, bottom, bottomTrailing).",
        defaultValue: "topLeading"
    )
    var contentAlignment: Alignment = .topLeading

    @BlockSlot(description: "Slot for composing child content within the stack.")
    var content: (BlockIndex) -> Content

    var body: some View {
        SwiftUI.ZStack(alignment: contentAlignment) {
            content(-1)
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

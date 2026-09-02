import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Box",
    keyType: "nativeblocks/box",
    description: "Stacking layout container; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct Box<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Children alignment (topStart, topCenter, topEnd, centerStart, center, centerEnd, bottomStart, bottomCenter, bottomEnd).",
        defaultValue: "topStart"
    )
    var contentAlignment: Alignment = .topLeading

    @BlockSlot(description: "Slot for composing child content within the box.")
    var content: (BlockIndex) -> Content

    var body: some View {
        ZStack(alignment: contentAlignment) {
            content(-1)
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

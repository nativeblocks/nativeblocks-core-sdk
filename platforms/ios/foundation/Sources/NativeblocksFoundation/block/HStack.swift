import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "HStack",
    keyType: "nativeblocks/hstack",
    description: "Horizontal layout container; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct HStack<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Vertical alignment of children (top, center, bottom).",
        defaultValue: "top"
    )
    var verticalAlignment: VerticalAlignment = .top

    @BlockData(description: "The spacing between children.", defaultValue: "0")
    var spacing: CGFloat = 0

    @BlockSlot(description: "Slot for composing child content within the stack.")
    var content: (BlockIndex) -> Content

    var body: some View {
        SwiftUI.HStack(alignment: verticalAlignment, spacing: spacing) {
            content(-1)
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

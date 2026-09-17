import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "VStack",
    keyType: "nativeblocks/vstack",
    description: "Vertical layout container; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct VStack<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Horizontal alignment of children (leading, center, trailing).",
        defaultValue: "leading"
    )
    var horizontalAlignment: HorizontalAlignment = .leading

    @BlockData(description: "The spacing between children.", defaultValue: "0")
    var spacing: CGFloat = 0

    @BlockSlot(description: "Slot for composing child content within the stack.")
    var content: (BlockIndex) -> Content

    var body: some View {
        SwiftUI.VStack(alignment: horizontalAlignment, spacing: spacing) {
            content(-1)
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

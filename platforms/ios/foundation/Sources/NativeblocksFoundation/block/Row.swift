import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Row",
    keyType: "nativeblocks/row",
    description: "Horizontal layout container; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct Row<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Vertical alignment of children (top, bottom, center).",
        defaultValue: "top"
    )
    var verticalAlignment: VerticalAlignment = .top

    @BlockData(description: "The spacing between children.", defaultValue: "0")
    var spacing: CGFloat = 0

    @BlockSlot(description: "Slot for composing child content within the row.")
    var content: (BlockIndex) -> Content

    var body: some View {
        HStack(alignment: verticalAlignment, spacing: spacing) {
            content(-1)
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Lazy VStack",
    keyType: "nativeblocks/lazy_vstack",
    description: "Scrollable vertical list; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct LazyVStack<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Horizontal alignment of children (leading, center, trailing).",
        defaultValue: "leading"
    )
    var horizontalAlignment: HorizontalAlignment = .leading

    @BlockData(description: "The spacing between children.", defaultValue: "0")
    var spacing: CGFloat = 0

    @BlockSlot(description: "Slot describing what the list contains.", scope: "LIST")
    var content: (Any) -> Content

    var body: some View {
        ScrollView(.vertical) {
            SwiftUI.LazyVStack(alignment: horizontalAlignment, spacing: spacing) {
                content(blockContext?.scope ?? ())
            }
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

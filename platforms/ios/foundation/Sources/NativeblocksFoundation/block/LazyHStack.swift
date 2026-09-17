import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Lazy HStack",
    keyType: "nativeblocks/lazy_hstack",
    description: "Scrollable horizontal list; style it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct LazyHStack<Content: View>: View {
    var blockContext: BlockContext? = nil

    @BlockData(
        description: "Vertical alignment of children (top, center, bottom).",
        defaultValue: "top"
    )
    var verticalAlignment: VerticalAlignment = .top

    @BlockData(description: "The spacing between children.", defaultValue: "0")
    var spacing: CGFloat = 0

    @BlockSlot(description: "Slot describing what the list contains.", scope: "LIST")
    var content: (Any) -> Content

    var body: some View {
        ScrollView(.horizontal) {
            SwiftUI.LazyHStack(alignment: verticalAlignment, spacing: spacing) {
                content(blockContext?.scope ?? ())
            }
        }
        .modifier(blockContext?.modifier ?? .none)
    }
}

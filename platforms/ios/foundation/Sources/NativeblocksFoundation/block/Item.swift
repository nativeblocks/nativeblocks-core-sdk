import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Item",
    keyType: "nativeblocks/item",
    description: "One item of fixed content inside a list.",
    scope: "LIST",
    version: 1,
    versionName: "1"
)
struct Item<Content: View>: View {
    var blockContext: BlockContext? = nil

    var describeScope: Any = ()

    @BlockSlot(description: "The content of this item.")
    var content: (BlockIndex) -> Content

    var body: some View {
        return content(-1)
    }
}

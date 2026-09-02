import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Items",
    keyType: "nativeblocks/items",
    description: "One row per element of a list.",
    scope: "LIST",
    version: 1,
    versionName: "1"
)
struct Items<Content: View>: View {
    var blockContext: BlockContext? = nil

    var describeScope: Any = ()

    @BlockData(description: "The list to repeat over, as JSON.", defaultValue: "[]")
    var list: String = "[]"

    @BlockData(
        description: "Path to a value that tells rows apart, e.g. \"id\". Falls back to position.",
        defaultValue: ""
    )
    var id: String = ""

    @BlockData(
        description: "The element a row is built for, as JSON. Written per row by the runtime.",
        defaultValue: ""
    )
    var item: String = ""

    @BlockSlot(
        description: "Built once per element; receives the element it belongs to.",
        dataBindings: ["item"]
    )
    var content: (BlockIndex, Any) -> Content

    var body: some View {
        let rows = readListItems(list, id)
        return ForEach(Array(rows.enumerated()), id: \.element.id) { index, row in
            content(index, ListRowScope(root: blockContext?.block.key ?? "", element: row.value, parent: blockContext?.scope))
        }
    }
}

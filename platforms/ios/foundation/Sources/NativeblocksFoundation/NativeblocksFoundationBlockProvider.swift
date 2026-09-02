import NativeblocksRuntime
public class NativeblocksFoundationBlockProvider {
    public static func provideBlocks(name: String = "default") {
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/items",
            block: .describing { blockContext, describeScope in
                ItemsBlock(blockContext: blockContext, describeScope: describeScope)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/spacer",
            block: .rendering { blockContext in
                SpacerBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/item",
            block: .describing { blockContext, describeScope in
                ItemBlock(blockContext: blockContext, describeScope: describeScope)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/lazy_column",
            block: .rendering { blockContext in
                LazyColumnBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/column",
            block: .rendering { blockContext in
                ColumnBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/box",
            block: .rendering { blockContext in
                BoxBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/lazy_row",
            block: .rendering { blockContext in
                LazyRowBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/row",
            block: .rendering { blockContext in
                RowBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/text",
            block: .rendering { blockContext in
                TextBlock(blockContext: blockContext)
            }
        )
    }
}
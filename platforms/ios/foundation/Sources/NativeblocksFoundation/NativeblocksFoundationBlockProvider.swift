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
            blockType: "nativeblocks/lazy_vstack",
            block: .rendering { blockContext in
                LazyVStackBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/vstack",
            block: .rendering { blockContext in
                VStackBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/zstack",
            block: .rendering { blockContext in
                ZStackBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/lazy_hstack",
            block: .rendering { blockContext in
                LazyHStackBlock(blockContext: blockContext)
            }
        )
        NativeblocksManager.getInstance(name: name).provideBlock(
            blockType: "nativeblocks/hstack",
            block: .rendering { blockContext in
                HStackBlock(blockContext: blockContext)
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
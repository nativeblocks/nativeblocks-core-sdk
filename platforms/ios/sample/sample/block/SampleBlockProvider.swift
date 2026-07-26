import NativeblocksRuntime

public class SampleBlockProvider {
    public static func provideBlocks(name: String = "default") {
        NativeblocksManager.getInstance(name: name).provideBlock(blockType: "SAMPLE_TEXT") { props in
            SampleTextBlock(blockProps: props)
        }
    }
}

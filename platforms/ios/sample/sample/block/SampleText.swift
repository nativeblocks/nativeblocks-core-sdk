import SwiftUI
import NativeblocksCompiler
import NativeblocksRuntime

@Block(
    name: "Sample text",
    keyType: "SAMPLE_TEXT",
    description: "Renders a line of text supplied by the frame"
)
struct SampleText: View {
    
    var blockContext: BlockContext? = nil
    
    @BlockData(description: "Text to render")
    var text: String

    @BlockData(description: "Font size in points", defaultValue: "16")
    var fontSize: CGFloat = 16

    @BlockData(description: "Render the text in bold", defaultValue: "false")
    var bold: Bool = false

    var body: some View {
        Text(text)
            .font(.system(size: fontSize))
            .fontWeight(bold ? .bold : .regular)
            .modifier(blockContext?.modifier ?? .none)
    }
}

import SwiftUI
import NativeblocksCompiler
import NativeblocksRuntime

@NativeBlock(
    name: "Sample text",
    keyType: "SAMPLE_TEXT",
    description: "Renders a line of text supplied by the frame"
)
struct SampleText: View {

    @NativeBlockData(description: "Text to render")
    var text: String

    @NativeBlockProp(description: "Font size in points", defaultValue: "16")
    var fontSize: CGFloat = 16

    @NativeBlockProp(description: "Render the text in bold", defaultValue: "false")
    var bold: Bool = false

    var body: some View {
        Text(text)
            .font(.system(size: fontSize))
            .fontWeight(bold ? .bold : .regular)
    }
}

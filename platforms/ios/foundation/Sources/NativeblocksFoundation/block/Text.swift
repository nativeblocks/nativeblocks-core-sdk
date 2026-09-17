import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Text",
    keyType: "nativeblocks/text",
    description: "Text content; style the space around it by attaching modifiers.",
    version: 1,
    versionName: "1"
)
struct Text: View {
    var blockContext: BlockContext? = nil

    @BlockData(description: "The text content to display.")
    var text: String = ""

    @BlockData(description: "Font size in points.", defaultValue: "14.0")
    var fontSize: CGFloat = 14

    @BlockData(description: "Text color in hexadecimal format.", defaultValue: "#ff000000")
    var color: Color = .black

    @BlockData(
        description: "Font weight (thin, extraLight, light, normal, medium, semiBold, bold, extraBold, black).",
        defaultValue: "normal"
    )
    var fontWeight: Font.Weight = .regular

    @BlockData(description: "Text alignment (leading, center, trailing).", defaultValue: "leading")
    var textAlign: TextAlignment = .leading

    @BlockData(description: "Maximum number of lines to display.", defaultValue: "9999")
    var lineLimit: Int = 9999

    var body: some View {
        SwiftUI.Text(text)
            .font(.system(size: fontSize, weight: fontWeight))
            .foregroundColor(color)
            .multilineTextAlignment(textAlign)
            .lineLimit(lineLimit)
            .modifier(blockContext?.modifier ?? .none)
    }
}

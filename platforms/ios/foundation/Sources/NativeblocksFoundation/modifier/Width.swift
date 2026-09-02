import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Width",
    keyType: "nativeblocks/width",
    description: "Sets the width of the block.",
    version: 1,
    versionName: "1"
)
struct Width: ViewModifier {

    @ModifierData(description: "The width of the block ('match', 'wrap' or a number).", defaultValue: "wrap")
    var value: String = "wrap"

    @ViewBuilder
    func body(content: Content) -> some View {
        switch value {
        case "match":
            content.frame(maxWidth: .infinity)
        case "wrap":
            content
        default:
            if let width = Double(value) {
                content.frame(width: width)
            } else {
                content
            }
        }
    }
}

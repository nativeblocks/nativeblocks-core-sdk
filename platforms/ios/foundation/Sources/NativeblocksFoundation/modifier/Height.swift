import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Height",
    keyType: "nativeblocks/height",
    description: "Sets the height of the block.",
    version: 1,
    versionName: "1"
)
struct Height: ViewModifier {

    @ModifierData(description: "The height of the block ('match', 'wrap' or a number).", defaultValue: "wrap")
    var value: String = "wrap"

    @ViewBuilder
    func body(content: Content) -> some View {
        switch value {
        case "match":
            content.frame(maxHeight: .infinity)
        case "wrap":
            content
        default:
            if let height = Double(value) {
                content.frame(height: height)
            } else {
                content
            }
        }
    }
}

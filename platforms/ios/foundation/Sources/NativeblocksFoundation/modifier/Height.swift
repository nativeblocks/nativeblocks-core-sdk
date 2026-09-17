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

    @ModifierData(description: "The height of the block ('infinity', 'fit' or a number).", defaultValue: "fit")
    var value: String = "fit"

    @ViewBuilder
    func body(content: Content) -> some View {
        switch value {
        case "infinity":
            content.frame(maxHeight: .infinity)
        case "fit":
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

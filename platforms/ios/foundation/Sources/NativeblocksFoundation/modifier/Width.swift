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

    @ModifierData(description: "The width of the block ('infinity', 'fit' or a number).", defaultValue: "fit")
    var value: String = "fit"

    @ViewBuilder
    func body(content: Content) -> some View {
        switch value {
        case "infinity":
            content.frame(maxWidth: .infinity)
        case "fit":
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

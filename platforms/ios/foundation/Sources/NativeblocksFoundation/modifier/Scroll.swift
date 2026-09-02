import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Scroll",
    keyType: "nativeblocks/scroll",
    description: "Makes the block scrollable in one direction.",
    version: 1,
    versionName: "1"
)
struct Scroll: ViewModifier {

    @ModifierData(description: "Scroll direction (vertical, horizontal).", defaultValue: "vertical")
    var direction: String = "vertical"

    @ViewBuilder
    func body(content: Content) -> some View {
        switch direction {
        case "horizontal":
            ScrollView(.horizontal) { content }
        case "vertical":
            ScrollView(.vertical) { content }
        default:
            content
        }
    }
}

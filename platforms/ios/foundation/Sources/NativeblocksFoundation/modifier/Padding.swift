import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Padding",
    keyType: "nativeblocks/padding",
    description: "Adds padding around the block.",
    version: 1,
    versionName: "1"
)
struct Padding: ViewModifier {

    @ModifierData(description: "Padding on the start side.", defaultValue: "0.0")
    var start: CGFloat = 0

    @ModifierData(description: "Padding on the top side.", defaultValue: "0.0")
    var top: CGFloat = 0

    @ModifierData(description: "Padding on the end side.", defaultValue: "0.0")
    var end: CGFloat = 0

    @ModifierData(description: "Padding on the bottom side.", defaultValue: "0.0")
    var bottom: CGFloat = 0

    func body(content: Content) -> some View {
        content
            .padding(.leading, start)
            .padding(.top, top)
            .padding(.trailing, end)
            .padding(.bottom, bottom)
    }
}

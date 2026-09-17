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

    @ModifierData(description: "Padding on the leading side.", defaultValue: "0.0")
    var leading: CGFloat = 0

    @ModifierData(description: "Padding on the top side.", defaultValue: "0.0")
    var top: CGFloat = 0

    @ModifierData(description: "Padding on the trailing side.", defaultValue: "0.0")
    var trailing: CGFloat = 0

    @ModifierData(description: "Padding on the bottom side.", defaultValue: "0.0")
    var bottom: CGFloat = 0

    func body(content: Content) -> some View {
        content
            .padding(.leading, leading)
            .padding(.top, top)
            .padding(.trailing, trailing)
            .padding(.bottom, bottom)
    }
}

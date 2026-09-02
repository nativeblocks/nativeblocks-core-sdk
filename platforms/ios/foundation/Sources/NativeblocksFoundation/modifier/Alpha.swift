import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Alpha",
    keyType: "nativeblocks/alpha",
    description: "Sets the opacity of the block.",
    version: 1,
    versionName: "1"
)
struct Alpha: ViewModifier {

    @ModifierData(description: "Opacity between 0.0 (transparent) and 1.0 (opaque).", defaultValue: "1.0")
    var value: Double = 1

    func body(content: Content) -> some View {
        content.opacity(min(max(value, 0), 1))
    }
}

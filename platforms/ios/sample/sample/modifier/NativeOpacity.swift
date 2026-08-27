import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Opacity",
    keyType: "nativeblocks/opacity",
    description: "Fades the block it is attached to."
)
struct NativeOpacity: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @ModifierData(description: "Opacity between 0 and 1.", defaultValue: "1")
    var opacity: Double = 1

    func body(content: Content) -> some View {
        content.opacity(opacity)
    }
}

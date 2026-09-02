import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Rotate",
    keyType: "nativeblocks/rotate",
    description: "Rotates the block around its center.",
    version: 1,
    versionName: "1"
)
struct Rotate: ViewModifier {

    @ModifierData(description: "Rotation in degrees, clockwise.", defaultValue: "0.0")
    var degrees: Double = 0

    func body(content: Content) -> some View {
        content.rotationEffect(.degrees(degrees))
    }
}

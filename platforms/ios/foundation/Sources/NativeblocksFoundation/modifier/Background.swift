import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Background",
    keyType: "nativeblocks/background",
    description: "Fills the block background with a color.",
    version: 1,
    versionName: "1"
)
struct Background: ViewModifier {

    @ModifierData(description: "Background color in hexadecimal format.", defaultValue: "#00000000")
    var color: Color = Color.black.opacity(0)

    @ModifierData(description: "Shape of the background (rectangle, circle).", defaultValue: "rectangle")
    var shape: String = SHAPE_RECTANGLE

    @ModifierData(description: "Top-start corner radius.", defaultValue: "0.0")
    var radiusTopStart: CGFloat = 0

    @ModifierData(description: "Top-end corner radius.", defaultValue: "0.0")
    var radiusTopEnd: CGFloat = 0

    @ModifierData(description: "Bottom-start corner radius.", defaultValue: "0.0")
    var radiusBottomStart: CGFloat = 0

    @ModifierData(description: "Bottom-end corner radius.", defaultValue: "0.0")
    var radiusBottomEnd: CGFloat = 0

    @Environment(\.layoutDirection) var layoutDirection

    func body(content: Content) -> some View {
        content.background(
            CornerShape(
                shape: shape,
                topStart: radiusTopStart,
                topEnd: radiusTopEnd,
                bottomStart: radiusBottomStart,
                bottomEnd: radiusBottomEnd,
                layoutDirection: layoutDirection
            ).fill(color)
        )
    }
}

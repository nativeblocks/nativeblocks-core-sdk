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

    @ModifierData(description: "Top-leading corner radius.", defaultValue: "0.0")
    var radiusTopLeading: CGFloat = 0

    @ModifierData(description: "Top-trailing corner radius.", defaultValue: "0.0")
    var radiusTopTrailing: CGFloat = 0

    @ModifierData(description: "Bottom-leading corner radius.", defaultValue: "0.0")
    var radiusBottomLeading: CGFloat = 0

    @ModifierData(description: "Bottom-trailing corner radius.", defaultValue: "0.0")
    var radiusBottomTrailing: CGFloat = 0

    @Environment(\.layoutDirection) var layoutDirection

    func body(content: Content) -> some View {
        content.background(
            CornerShape(
                shape: shape,
                topLeading: radiusTopLeading,
                topTrailing: radiusTopTrailing,
                bottomLeading: radiusBottomLeading,
                bottomTrailing: radiusBottomTrailing,
                layoutDirection: layoutDirection
            ).fill(color)
        )
    }
}

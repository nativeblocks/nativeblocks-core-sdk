import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Clip",
    keyType: "nativeblocks/clip",
    description: "Clips the block content to a shape.",
    version: 1,
    versionName: "1"
)
struct Clip: ViewModifier {

    @ModifierData(description: "Shape to clip to (rectangle, circle).", defaultValue: "rectangle")
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
        content.clipShape(
            CornerShape(
                shape: shape,
                topLeading: radiusTopLeading,
                topTrailing: radiusTopTrailing,
                bottomLeading: radiusBottomLeading,
                bottomTrailing: radiusBottomTrailing,
                layoutDirection: layoutDirection
            )
        )
    }
}

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
        content.clipShape(
            CornerShape(
                shape: shape,
                topStart: radiusTopStart,
                topEnd: radiusTopEnd,
                bottomStart: radiusBottomStart,
                bottomEnd: radiusBottomEnd,
                layoutDirection: layoutDirection
            )
        )
    }
}

import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Shadow",
    keyType: "nativeblocks/shadow",
    description: "Drops a shadow behind the block.",
    version: 1,
    versionName: "1"
)
struct Shadow: ViewModifier {

    @ModifierData(description: "Shadow elevation.", defaultValue: "0.0")
    var elevation: CGFloat = 0

    @ModifierData(description: "Shape of the shadow (rectangle, circle).", defaultValue: "rectangle")
    var shape: String = SHAPE_RECTANGLE

    @ModifierData(description: "Top-start corner radius.", defaultValue: "0.0")
    var radiusTopStart: CGFloat = 0

    @ModifierData(description: "Top-end corner radius.", defaultValue: "0.0")
    var radiusTopEnd: CGFloat = 0

    @ModifierData(description: "Bottom-start corner radius.", defaultValue: "0.0")
    var radiusBottomStart: CGFloat = 0

    @ModifierData(description: "Bottom-end corner radius.", defaultValue: "0.0")
    var radiusBottomEnd: CGFloat = 0

    @ModifierData(description: "Whether the content is also clipped to the shape.", defaultValue: "true")
    var clipToShape: Bool = true

    @Environment(\.layoutDirection) var layoutDirection

    @ViewBuilder
    func body(content: Content) -> some View {
        if clipToShape {
            content
                .clipShape(
                    CornerShape(
                        shape: shape,
                        topStart: radiusTopStart,
                        topEnd: radiusTopEnd,
                        bottomStart: radiusBottomStart,
                        bottomEnd: radiusBottomEnd,
                        layoutDirection: layoutDirection
                    )
                )
                .shadow(radius: elevation, y: elevation / 2)
        } else {
            content.shadow(radius: elevation, y: elevation / 2)
        }
    }
}

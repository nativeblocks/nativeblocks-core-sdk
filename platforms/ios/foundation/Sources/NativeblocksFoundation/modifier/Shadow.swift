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

    @ModifierData(description: "Shadow blur radius.", defaultValue: "0.0")
    var radius: CGFloat = 0

    @ModifierData(description: "Shape of the shadow (rectangle, circle).", defaultValue: "rectangle")
    var shape: String = SHAPE_RECTANGLE

    @ModifierData(description: "Top-leading corner radius.", defaultValue: "0.0")
    var radiusTopLeading: CGFloat = 0

    @ModifierData(description: "Top-trailing corner radius.", defaultValue: "0.0")
    var radiusTopTrailing: CGFloat = 0

    @ModifierData(description: "Bottom-leading corner radius.", defaultValue: "0.0")
    var radiusBottomLeading: CGFloat = 0

    @ModifierData(description: "Bottom-trailing corner radius.", defaultValue: "0.0")
    var radiusBottomTrailing: CGFloat = 0

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
                        topLeading: radiusTopLeading,
                        topTrailing: radiusTopTrailing,
                        bottomLeading: radiusBottomLeading,
                        bottomTrailing: radiusBottomTrailing,
                        layoutDirection: layoutDirection
                    )
                )
                .shadow(radius: radius, y: radius / 2)
        } else {
            content.shadow(radius: radius, y: radius / 2)
        }
    }
}

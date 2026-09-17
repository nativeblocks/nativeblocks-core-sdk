import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Border",
    keyType: "nativeblocks/border",
    description: "Draws a border around the block.",
    version: 1,
    versionName: "1"
)
struct Border: ViewModifier {

    @ModifierData(description: "Border width.", defaultValue: "1.0")
    var width: CGFloat = 1

    @ModifierData(description: "Border color in hexadecimal format.", defaultValue: "#00000000")
    var color: Color = Color.black.opacity(0)

    @ModifierData(description: "Shape of the border (rectangle, circle).", defaultValue: "rectangle")
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
        let outline = CornerShape(
            shape: shape,
            topLeading: radiusTopLeading,
            topTrailing: radiusTopTrailing,
            bottomLeading: radiusBottomLeading,
            bottomTrailing: radiusBottomTrailing,
            layoutDirection: layoutDirection
        )
        // a stroke straddles the path, so draw it double width and clip the outer half away
        return content.overlay(outline.stroke(color, lineWidth: width * 2).clipShape(outline))
    }
}

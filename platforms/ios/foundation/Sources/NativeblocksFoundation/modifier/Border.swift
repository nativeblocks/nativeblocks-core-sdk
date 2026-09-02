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
        let outline = CornerShape(
            shape: shape,
            topStart: radiusTopStart,
            topEnd: radiusTopEnd,
            bottomStart: radiusBottomStart,
            bottomEnd: radiusBottomEnd,
            layoutDirection: layoutDirection
        )
        // a stroke straddles the path, so draw it double width and clip the outer half away
        return content.overlay(outline.stroke(color, lineWidth: width * 2).clipShape(outline))
    }
}

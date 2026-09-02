import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Gradient",
    keyType: "nativeblocks/gradient",
    description: "Fills the block background with a gradient of colors.",
    version: 1,
    versionName: "1"
)
struct Gradient: ViewModifier {

    @ModifierData(description: "Comma-separated hexadecimal colors (e.g. '#004FF0, #00FFFFFF').", defaultValue: "")
    var colors: String = ""

    @ModifierData(description: "Gradient type (linear, radial, sweep).", defaultValue: "linear")
    var type: String = GRADIENT_LINEAR

    @ModifierData(description: "Angle in degrees for linear gradients; 0 flows start-to-end, 90 top-to-bottom.", defaultValue: "0.0")
    var angle: Double = 0

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

    @ViewBuilder
    func body(content: Content) -> some View {
        let colorList = parseColors(colors)
        let outline = CornerShape(
            shape: shape,
            topStart: radiusTopStart,
            topEnd: radiusTopEnd,
            bottomStart: radiusBottomStart,
            bottomEnd: radiusBottomEnd,
            layoutDirection: layoutDirection
        )
        if colorList.isEmpty {
            content
        } else if colorList.count == 1 {
            content.background(outline.fill(colorList[0]))
        } else {
            content.background(outline.fill(gradientStyle(type, colorList, angle)))
        }
    }
}

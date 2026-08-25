import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@NativeModifier(
    name: "Shape",
    keyType: "nativeblocks/shape",
    description: "Clips the block to a shape and paints its background, border and shadow."
)
struct NativeShape: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @NativeModifierData(
        description: "Outline of the shape: rectangle, roundedRectangle, circle or capsule.",
        defaultValue: "rectangle"
    )
    var style: ShapeStyleType = .rectangle

    @NativeModifierData(description: "Top-leading corner radius.", defaultValue: "0")
    var radiusTopLeading: CGFloat = 0

    @NativeModifierData(description: "Top-trailing corner radius.", defaultValue: "0")
    var radiusTopTrailing: CGFloat = 0

    @NativeModifierData(description: "Bottom-leading corner radius.", defaultValue: "0")
    var radiusBottomLeading: CGFloat = 0

    @NativeModifierData(description: "Bottom-trailing corner radius.", defaultValue: "0")
    var radiusBottomTrailing: CGFloat = 0

    @NativeModifierData(description: "Background color in hex.", defaultValue: "#00000000")
    var backgroundColor: Color = Color.black.opacity(0)

    @NativeModifierData(description: "Border color in hex.", defaultValue: "#00000000")
    var borderColor: Color = Color.black.opacity(0)

    @NativeModifierData(description: "Border width.", defaultValue: "0")
    var borderWidth: CGFloat = 0

    @NativeModifierData(description: "Shadow radius.", defaultValue: "0")
    var shadowRadius: CGFloat = 0

    @NativeModifierData(description: "Clip the block's own content to the shape.", defaultValue: "true")
    var clipContent: Bool = true

    func body(content: Content) -> some View {
        let outline = ShapeOutline(
            style: style,
            topLeading: radiusTopLeading,
            topTrailing: radiusTopTrailing,
            bottomLeading: radiusBottomLeading,
            bottomTrailing: radiusBottomTrailing
        )
        return content
            .background(outline.fill(backgroundColor))
            .overlay(outline.stroke(borderColor, lineWidth: borderWidth))
            .clipShape(clipContent ? outline : ShapeOutline(style: .rectangle))
            .shadow(radius: shadowRadius)
    }
}

/// One concrete shape covering every `ShapeStyleType`, so the modifier needs no type erasure.
struct ShapeOutline: Shape {
    var style: ShapeStyleType
    var topLeading: CGFloat = 0
    var topTrailing: CGFloat = 0
    var bottomLeading: CGFloat = 0
    var bottomTrailing: CGFloat = 0

    func path(in rect: CGRect) -> Path {
        switch style {
        case .rectangle:
            return Path(rect)
        case .circle:
            let diameter = min(rect.width, rect.height)
            return Path(
                ellipseIn: CGRect(
                    x: rect.midX - diameter / 2,
                    y: rect.midY - diameter / 2,
                    width: diameter,
                    height: diameter
                )
            )
        case .capsule:
            let radius = min(rect.width, rect.height) / 2
            return rounded(rect, radius, radius, radius, radius)
        case .roundedRectangle:
            return rounded(rect, topLeading, topTrailing, bottomTrailing, bottomLeading)
        }
    }

    private func rounded(
        _ rect: CGRect, _ tl: CGFloat, _ tr: CGFloat, _ br: CGFloat, _ bl: CGFloat
    ) -> Path {
        let limit = min(rect.width, rect.height) / 2
        let tl = min(max(tl, 0), limit)
        let tr = min(max(tr, 0), limit)
        let br = min(max(br, 0), limit)
        let bl = min(max(bl, 0), limit)

        var path = Path()
        path.move(to: CGPoint(x: rect.minX + tl, y: rect.minY))
        path.addLine(to: CGPoint(x: rect.maxX - tr, y: rect.minY))
        path.addArc(
            center: CGPoint(x: rect.maxX - tr, y: rect.minY + tr), radius: tr,
            startAngle: .degrees(-90), endAngle: .degrees(0), clockwise: false)
        path.addLine(to: CGPoint(x: rect.maxX, y: rect.maxY - br))
        path.addArc(
            center: CGPoint(x: rect.maxX - br, y: rect.maxY - br), radius: br,
            startAngle: .degrees(0), endAngle: .degrees(90), clockwise: false)
        path.addLine(to: CGPoint(x: rect.minX + bl, y: rect.maxY))
        path.addArc(
            center: CGPoint(x: rect.minX + bl, y: rect.maxY - bl), radius: bl,
            startAngle: .degrees(90), endAngle: .degrees(180), clockwise: false)
        path.addLine(to: CGPoint(x: rect.minX, y: rect.minY + tl))
        path.addArc(
            center: CGPoint(x: rect.minX + tl, y: rect.minY + tl), radius: tl,
            startAngle: .degrees(180), endAngle: .degrees(270), clockwise: false)
        path.closeSubpath()
        return path
    }
}

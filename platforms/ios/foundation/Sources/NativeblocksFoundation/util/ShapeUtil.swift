import SwiftUI

let SHAPE_RECTANGLE = "rectangle"
let SHAPE_CIRCLE = "circle"

/// One concrete shape covering every shape keyword, so a modifier needs no type erasure.
///
/// Radii are named after the reading direction, the way a frame authors them, and only
/// apply to `SHAPE_RECTANGLE`; all-zero radii resolve to a plain rectangle.
struct CornerShape: Shape {
    var shape: String = SHAPE_RECTANGLE
    var topLeading: CGFloat = 0
    var topTrailing: CGFloat = 0
    var bottomLeading: CGFloat = 0
    var bottomTrailing: CGFloat = 0
    var layoutDirection: LayoutDirection = .leftToRight

    func path(in rect: CGRect) -> Path {
        if shape == SHAPE_CIRCLE {
            let diameter = min(rect.width, rect.height)
            return Path(
                ellipseIn: CGRect(
                    x: rect.midX - diameter / 2,
                    y: rect.midY - diameter / 2,
                    width: diameter,
                    height: diameter
                )
            )
        }

        let limit = min(rect.width, rect.height) / 2
        let clamp = { (radius: CGFloat) in min(max(radius, 0), limit) }
        let mirrored = layoutDirection == .rightToLeft
        let tl = clamp(mirrored ? topTrailing : topLeading)
        let tr = clamp(mirrored ? topLeading : topTrailing)
        let bl = clamp(mirrored ? bottomTrailing : bottomLeading)
        let br = clamp(mirrored ? bottomLeading : bottomTrailing)

        if tl == 0 && tr == 0 && bl == 0 && br == 0 {
            return Path(rect)
        }

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

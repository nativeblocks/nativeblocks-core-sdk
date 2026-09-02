import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Aspect Ratio",
    keyType: "nativeblocks/aspect_ratio",
    description: "Sizes the block to a width/height ratio.",
    version: 1,
    versionName: "1"
)
struct AspectRatio: ViewModifier {

    @ModifierData(description: "Width to height ratio (e.g. 1.777 for 16:9); must be greater than 0.", defaultValue: "1.0")
    var ratio: CGFloat = 1

    @ModifierData(description: "Whether the block fills the space offered to it rather than fitting inside it.", defaultValue: "false")
    var fill: Bool = false

    @ViewBuilder
    func body(content: Content) -> some View {
        // aspectRatio draws nothing sensible for a non-positive ratio
        if ratio <= 0 {
            content
        } else {
            content.aspectRatio(ratio, contentMode: fill ? .fill : .fit)
        }
    }
}

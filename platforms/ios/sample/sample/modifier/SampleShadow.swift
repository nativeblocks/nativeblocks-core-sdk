import SwiftUI
import NativeblocksCompiler
import NativeblocksRuntime

@NativeModifier(
    name: "Sample shadow",
    keyType: "SAMPLE_SHADOW",
    description: "Drops a rounded shadow behind the block it is attached to"
)
struct SampleShadow: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @NativeModifierData(description: "Shadow radius in points", defaultValue: "4")
    var radius: CGFloat = 4

    @NativeModifierData(description: "Corner radius in points", defaultValue: "8")
    var cornerRadius: CGFloat = 8

    func body(content: Content) -> some View {
        content
            .clipShape(RoundedRectangle(cornerRadius: cornerRadius))
            .shadow(radius: radius)
    }
}

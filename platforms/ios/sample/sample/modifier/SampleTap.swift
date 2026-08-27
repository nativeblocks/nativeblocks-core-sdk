import SwiftUI
import NativeblocksCompiler
import NativeblocksRuntime

@Modifier(
    name: "Sample tap",
    keyType: "SAMPLE_TAP",
    description: "Makes the block it is attached to tappable"
)
struct SampleTap: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @ModifierData(description: "Whether the tap is enabled", defaultValue: "true")
    var enabled: Bool = true

    @ModifierEvent(description: "Triggered when the block is tapped")
    var onTap: (() -> Void)? = nil

    func body(content: Content) -> some View {
        content.onTapGesture {
            if enabled { onTap?() }
        }
    }
}

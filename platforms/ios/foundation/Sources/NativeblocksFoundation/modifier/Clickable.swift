import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Clickable",
    keyType: "nativeblocks/clickable",
    description: "Makes the block tappable (and optionally long-pressable).",
    version: 1,
    versionName: "1"
)
struct Clickable: ViewModifier {

    @ModifierData(description: "Whether tap handling is enabled.", defaultValue: "true")
    var enabled: Bool = true

    @ModifierData(description: "Whether a haptic feedback fires on long press.", defaultValue: "true")
    var hapticsOnLongClick: Bool = true

    @ModifierEvent(description: "Triggered when the block is tapped.")
    var onClick: (() -> Void)? = nil

    @ModifierEvent(description: "Triggered when the block is long-pressed.")
    var onLongClick: (() -> Void)? = nil

    @ViewBuilder
    func body(content: Content) -> some View {
        if enabled, onClick != nil || onLongClick != nil {
            let tappable = content
                .contentShape(Rectangle())
                .onTapGesture { onClick?() }
            if let onLongClick = onLongClick {
                tappable.onLongPressGesture {
                    if hapticsOnLongClick { performLongPressHaptics() }
                    onLongClick()
                }
            } else {
                tappable
            }
        } else {
            content
        }
    }
}

private func performLongPressHaptics() {
    #if os(iOS)
        UIImpactFeedbackGenerator(style: .medium).impactOccurred()
    #endif
}

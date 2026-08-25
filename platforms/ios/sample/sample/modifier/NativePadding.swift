import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@NativeModifier(
    name: "Padding",
    keyType: "nativeblocks/padding",
    description: "Applies padding on each side of the block."
)
struct NativePadding: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @NativeModifierData(description: "Padding on the leading side.", defaultValue: "0")
    var paddingLeading: CGFloat = 0

    @NativeModifierData(description: "Padding on the top side.", defaultValue: "0")
    var paddingTop: CGFloat = 0

    @NativeModifierData(description: "Padding on the trailing side.", defaultValue: "0")
    var paddingTrailing: CGFloat = 0

    @NativeModifierData(description: "Padding on the bottom side.", defaultValue: "0")
    var paddingBottom: CGFloat = 0

    func body(content: Content) -> some View {
        content
            .padding(.leading, paddingLeading)
            .padding(.top, paddingTop)
            .padding(.trailing, paddingTrailing)
            .padding(.bottom, paddingBottom)
    }
}

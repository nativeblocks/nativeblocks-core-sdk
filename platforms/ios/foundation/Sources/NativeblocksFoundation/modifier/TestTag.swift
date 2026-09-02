import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Test Tag",
    keyType: "nativeblocks/test_tag",
    description: "Tags the block for UI tests (Maestro).",
    version: 1,
    versionName: "1"
)
struct TestTag: ViewModifier {

    @ModifierData(description: "The tag used by UI test selectors.", defaultValue: "")
    var tag: String = ""

    @ViewBuilder
    func body(content: Content) -> some View {
        if tag.isEmpty {
            content
        } else {
            content.accessibilityIdentifier(tag)
        }
    }
}

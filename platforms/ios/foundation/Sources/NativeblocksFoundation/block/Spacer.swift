import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Block(
    name: "Spacer",
    keyType: "nativeblocks/spacer",
    description: "Empty space; size it by attaching width/height modifiers.",
    version: 1,
    versionName: "1"
)
struct Spacer: View {
    var blockContext: BlockContext? = nil

    var body: some View {
        SwiftUI.Spacer(minLength: 0)
            .modifier(blockContext?.modifier ?? .none)
    }
}

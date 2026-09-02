import NativeblocksCompiler
import NativeblocksRuntime
import SwiftUI

@Modifier(
    name: "Paginate",
    keyType: "nativeblocks/paginate",
    description: "Fires an event when the list nears its end; attach it to the content of a lazy row/column.",
    version: 1,
    versionName: "1"
)
struct Paginate: ViewModifier {

    var modifierContext: ModifierContext? = nil

    @ModifierData(description: "Total number of items in the list; bind the same value as the list length.", defaultValue: "0")
    var length: Int = 0

    @ModifierData(description: "Current page; the event reports this value plus one.", defaultValue: "1")
    var currentPage: Int = 1

    @ModifierData(description: "Whether there are more pages to load.", defaultValue: "true")
    var hasNextPage: Bool = true

    @ModifierData(description: "How many items before the end of the list the event fires.", defaultValue: "0")
    var threshold: Int = 0

    @ModifierEvent(
        description: "Fired when the end of the list is reached; reports the next page to fetch.",
        dataBindings: ["currentPage"]
    )
    var onNextPage: ((Int) -> Void)? = nil

    func body(content: Content) -> some View {
        let index = modifierContext?.listItemIndex ?? NONE_INDEX
        let reachedEnd = index != NONE_INDEX && length > 0 && index >= length - 1 - max(threshold, 0)
        return content.task(id: "\(index)-\(length)-\(currentPage)-\(hasNextPage)") {
            if reachedEnd && hasNextPage {
                onNextPage?(currentPage + 1)
            }
        }
    }
}

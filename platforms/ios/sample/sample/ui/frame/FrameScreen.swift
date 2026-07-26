import SwiftUI
import NativeblocksRuntime

struct FrameScreen: View {
    let instance: String
    let route: String
    let title: String

    var body: some View {
        NativeblocksFrame(
            instance: instance,
            route: route,
            routeArguments: [:],
            loading: { AnyView(NativeblocksLoading()) },
            error: { message in AnyView(NativeblocksError(message: message)) }
        )
        .navigationTitle(title)
        .navigationBarTitleDisplayMode(.inline)
    }
}

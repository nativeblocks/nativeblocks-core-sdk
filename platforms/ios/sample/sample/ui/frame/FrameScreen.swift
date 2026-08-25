import SwiftUI
import NativeblocksRuntime

struct FrameScreen: View {
    static let screenIdentifier = "frame-screen"
    static let loadingIdentifier = "frame-loading"
    static let errorIdentifier = "frame-error"

    let instanceName: String
    let route: String
    let title: String

    var body: some View {
        NativeblocksFrame(
            instanceName: instanceName,
            route: route,
            routeArguments: [:],
            loading: {
                AnyView(
                    NativeblocksLoading()
                        .accessibilityElement(children: .contain)
                        .accessibilityIdentifier(Self.loadingIdentifier)
                )
            },
            error: { message in
                AnyView(
                    NativeblocksError(message: message)
                        .accessibilityElement(children: .contain)
                        .accessibilityIdentifier(Self.errorIdentifier)
                )
            }
        )
        .accessibilityElement(children: .contain)
        .accessibilityIdentifier(Self.screenIdentifier)
        .navigationTitle(title)
        .navigationBarTitleDisplayMode(.inline)
    }
}

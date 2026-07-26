import SwiftUI

struct SampleNavigationStack: View {
    let instanceManager: InstanceManager

    @State private var path: [Destination] = []

    var body: some View {
        NavigationStack(path: $path) {
            InstanceListScreen(
                instanceManager: instanceManager,
                onInstanceOpened: { instance in
                    path.append(.frames(instance: instance.instanceKey))
                }
            )
            .navigationDestination(for: Destination.self) { destination in
                switch destination {
                case .frames(let instance):
                    FrameListScreen(
                        instance: instance,
                        onFrameOpened: { frame in
                            path.append(
                                .frame(
                                    instance: instance,
                                    route: frame.route,
                                    title: frame.name
                                )
                            )
                        }
                    )
                case .frame(let instance, let route, let title):
                    FrameScreen(instance: instance, route: route, title: title)
                }
            }
        }
    }
}

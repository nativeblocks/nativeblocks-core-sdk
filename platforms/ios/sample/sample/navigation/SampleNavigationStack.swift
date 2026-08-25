import SwiftUI

struct SampleNavigationStack: View {
    let instanceManager: InstanceManager
    var launchArguments = LaunchArguments()

    @State private var path: [Destination] = []

    var body: some View {
        NavigationStack(path: $path) {
            InstanceListScreen(
                instanceManager: instanceManager,
                launchArguments: launchArguments,
                onInstanceOpened: { instance in
                    path.append(.frames(instance: instance.instanceKey))
                },
                onRouteOpened: openRoute
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
                        },
                        onPreviewKitLaunched: previewKitLauncher(for: instance)
                    )
                case .frame(let instance, let route, let title):
                    FrameScreen(instanceName: instance, route: route, title: title)
                }
            }
        }
        .task { openLaunchArgumentRoute() }
    }

    /// PreviewKit is only attached to production instances, so the toolbar entry
    /// point only exists there.
    private func previewKitLauncher(for instanceKey: String) -> (() -> Void)? {
        guard let instance = instanceManager.instances.first(where: { $0.instanceKey == instanceKey }),
            !instance.developmentMode
        else { return nil }
        return { instanceManager.launchPreviewKit(instance) }
    }

    private func openLaunchArgumentRoute() {
        guard path.isEmpty, let route = launchArguments.route else { return }
        guard let instance = launchArguments.resolvedInstance(in: instanceManager.instances) else {
            return
        }
        openRoute(instance, route)
    }

    /// Pushes the frame straight onto the root, skipping the frame list. That list
    /// fetches the scaffold over the network every time it appears, which would put
    /// a network round trip inside any measurement that navigates back and forth.
    private func openRoute(_ instance: InstanceInfo, _ route: String) {
        instanceManager.start(instance)
        path = [.frame(instance: instance.instanceKey, route: route, title: route)]
    }
}

import SwiftUI

struct InstanceListScreen: View {
    static let directRouteIdentifier = "open-route-directly"

    let instanceManager: InstanceManager
    var launchArguments = LaunchArguments()
    let onInstanceOpened: (InstanceInfo) -> Void
    var onRouteOpened: (InstanceInfo, String) -> Void = { _, _ in }

    private var directRoute: (instance: InstanceInfo, route: String)? {
        guard let route = launchArguments.route,
            let instance = launchArguments.resolvedInstance(in: instanceManager.instances)
        else { return nil }
        return (instance, route)
    }

    var body: some View {
        List {
            if let directRoute {
                Button("Open \(directRoute.route)") {
                    onRouteOpened(directRoute.instance, directRoute.route)
                }
                .accessibilityIdentifier(Self.directRouteIdentifier)
            }

            ForEach(instanceManager.instances, id: \.instanceKey) { instance in
                Button {
                    instanceManager.start(instance)
                    onInstanceOpened(instance)
                } label: {
                    VStack(alignment: .leading, spacing: 4) {
                        Text(instance.name)
                        Text(instance.developmentMode ? "development" : "production")
                            .font(.footnote)
                            .foregroundStyle(.secondary)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .contentShape(.rect)
                }
                .buttonStyle(.plain)
            }
        }
        .navigationTitle("Instances")
    }
}

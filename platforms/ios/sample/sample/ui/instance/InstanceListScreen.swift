import SwiftUI

struct InstanceListScreen: View {
    let instanceManager: InstanceManager
    let onInstanceOpened: (InstanceInfo) -> Void

    var body: some View {
        List(instanceManager.instances, id: \.instanceKey) { instance in
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
        .navigationTitle("Instances")
    }
}

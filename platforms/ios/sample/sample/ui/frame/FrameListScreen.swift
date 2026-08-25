import SwiftUI
import NativeblocksRuntime

private enum ScaffoldState {
    case loading
    case ready(frames: [FrameRoute])
    case failed(message: String)
}

struct FrameListScreen: View {
    let instance: String
    let onFrameOpened: (FrameRoute) -> Void
    var onPreviewKitLaunched: (() -> Void)? = nil

    @State private var state: ScaffoldState = .loading

    var body: some View {
        Group {
            switch state {
            case .loading:
                ProgressView()
                    .frame(maxWidth: .infinity, maxHeight: .infinity)

            case .failed(let message):
                Text(message)
                    .padding(24)
                    .frame(maxWidth: .infinity, maxHeight: .infinity)

            case .ready(let frames):
                if frames.isEmpty {
                    Text("No frames in this scaffold")
                        .frame(maxWidth: .infinity, maxHeight: .infinity)
                } else {
                    List(frames, id: \.route) { frame in
                        Button {
                            onFrameOpened(frame)
                        } label: {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(frame.name)
                                Text(frame.route)
                                    .font(.footnote)
                                    .foregroundStyle(.secondary)
                            }
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .contentShape(.rect)
                        }
                        .buttonStyle(.plain)
                    }
                }
            }
        }
        .navigationTitle(instance)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            if let onPreviewKitLaunched {
                ToolbarItem(placement: .topBarTrailing) {
                    Button("Preview", systemImage: "slider.horizontal.3", action: onPreviewKitLaunched)
                }
            }
        }
        .task(id: instance) {
            state = .loading
            switch await NativeblocksManager.getInstance(name: instance).getScaffold() {
            case .success(let scaffold):
                state = .ready(frames: scaffold.frames.compactMap { $0.toFrameRoute() })
            case .failure(let error):
                state = .failed(message: error.localizedDescription)
            }
        }
    }
}

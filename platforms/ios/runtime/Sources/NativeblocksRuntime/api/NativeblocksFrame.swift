import Foundation
import SwiftUI

/// Represents a view that manages and displays a Nativeblocks frame.
/// The `NativeblocksFrame` is used to display a portion of the UI as defined by a specific route and its arguments.
public struct NativeblocksFrame: View {
    private var instanceName: String = "default"
    private var route: String
    private var routeArguments: [String: String]
    private var state: NativeblocksFrameState
    private var loading: () -> AnyView
    private var error: (String) -> AnyView

    /// Initializes a new `NativeblocksFrame` with the specified parameters.
    /// - Parameters:
    ///   - route: The route to be loaded.
    ///   - routeArguments: A dictionary of arguments to pass to the route.
    ///   - state: Whether the frame keeps what the user did to it, and under which key.
    ///   - loading: A closure that returns a view to be shown while the content is loading.
    ///   - error: A closure that takes a message string and returns a view to be shown when an error occurs.
    public init(
        instanceName: String = "default",
        route: String,
        routeArguments: [String: String],
        state: NativeblocksFrameState = .stateless,
        loading: @escaping () -> AnyView,
        error: @escaping (String) -> AnyView
    ) {
        self.instanceName = instanceName
        self.route = route
        self.routeArguments = routeArguments
        self.state = state
        self.loading = loading
        self.error = error
    }

    public var body: some View {
        let contractors = NativeblocksManager.getInstance(name: instanceName).providedActionContractors()
        ZStack {
            ForEach(Array(contractors.enumerated()), id: \.offset) { _, contractor in
                AnyView(contractor.actionContractor())
            }
            NativeFrame(
                instanceName: instanceName,
                route: route,
                args: routeArguments,
                state: state,
                loading: loading,
                error: error
            )
        }
    }
}

/// Represents a view displayed during the loading state of a `NativeblocksFrame`.
public struct NativeblocksLoading: View {
    public init() {}

    public var body: some View {
        #if os(iOS)
            let bgColor = Color(UIColor.systemBackground)
        #elseif os(macOS)
            let bgColor = Color(NSColor.windowBackgroundColor)
        #endif

        VStack {
            ProgressView()
                .progressViewStyle(CircularProgressViewStyle())
                .foregroundColor(Color.primary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(bgColor)
    }
}

/// Represents a view displayed when an error occurs while loading a `NativeblocksFrame`.
public struct NativeblocksError: View {
    private var message: String

    /// Initializes a new `NativeblocksError` view with the specified message.
    /// - Parameter message: The error message to display.
    public init(message: String) {
        self.message = message
    }

    public var body: some View {
        #if os(iOS)
            let bgColor = Color(UIColor.systemBackground)
        #elseif os(macOS)
            let bgColor = Color(NSColor.windowBackgroundColor)
        #endif

        VStack {
            Text("Something went wrong!")
                .foregroundColor(Color.primary)
            Text(message)
                .foregroundColor(Color.primary)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .background(bgColor)
    }
}

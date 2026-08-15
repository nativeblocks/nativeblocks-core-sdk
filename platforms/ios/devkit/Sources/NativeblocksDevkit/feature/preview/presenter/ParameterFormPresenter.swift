import Foundation
import Combine

#if os(iOS)
import UIKit
import SwiftUI

internal final class ParameterFormPresenter: ObservableObject {

    public static let shared = ParameterFormPresenter()

    @Published public var isPresented = false
    @Published public var instanceName: String?

    private var windowScene: UIWindowScene?
    private var overlayWindow: UIWindow?

    private init() {}

    public func present(instanceName: String) {
        guard overlayWindow == nil else { return }

        self.instanceName = instanceName

        guard let scene = UIApplication.shared.connectedScenes
            .compactMap({ $0 as? UIWindowScene })
            .first(where: { $0.activationState == .foregroundActive })
        else { return }

        windowScene = scene

        let window = UIWindow(windowScene: scene)
        window.windowLevel = .alert + 1
        window.backgroundColor = .clear

        let formView = ParameterFormView(
            instanceName: instanceName,
            onDismiss: { [weak self] in
                self?.dismiss()
            }
        )

        let hostingController = UIHostingController(rootView: formView)
        hostingController.view.backgroundColor = .clear

        window.rootViewController = hostingController
        window.makeKeyAndVisible()

        overlayWindow = window
        isPresented = true
    }

    public func dismiss() {
        overlayWindow?.isHidden = true
        overlayWindow = nil
        windowScene = nil
        instanceName = nil
        isPresented = false
    }
}

#else

internal final class ParameterFormPresenter: ObservableObject {

    public static let shared = ParameterFormPresenter()

    @Published public var isPresented = false
    @Published public var instanceName: String?

    private init() {}

    public func present(instanceName: String) {
        // Not implemented for macOS
    }

    public func dismiss() {
        isPresented = false
    }
}

#endif

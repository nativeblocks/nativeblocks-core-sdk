import SwiftUI
import UIKit

/// Shown when a frame references an action this app build does not ship.
internal func internalFallbackAction(instanceName: String, name: String) {
    let alert = UIAlertController(
        title: "Unsupported Action",
        message: "The \(name) action isn’t available in this app version",
        preferredStyle: .alert
    )
    alert.addAction(UIAlertAction(title: "OK", style: .default, handler: nil))
    alert.present(animated: true, completion: nil)
}

extension UIAlertController {
    internal func present(animated: Bool, completion: (() -> Void)?) {
        if let rootVC = UIApplication.shared.activeWindow?.rootViewController {
            presentFromController(controller: rootVC, animated: animated, completion: completion)
        }
    }

    private func presentFromController(controller: UIViewController, animated: Bool, completion: (() -> Void)?) {
        if let navVC = controller as? UINavigationController,
            let visibleVC = navVC.visibleViewController
        {
            presentFromController(controller: visibleVC, animated: animated, completion: completion)
        } else if let tabVC = controller as? UITabBarController,
            let selectedVC = tabVC.selectedViewController
        {
            presentFromController(controller: selectedVC, animated: animated, completion: completion)
        } else if let presented = controller.presentedViewController {
            presentFromController(controller: presented, animated: animated, completion: completion)
        } else {
            controller.present(self, animated: animated, completion: completion)
        }
    }
}

extension UIApplication {
    internal var activeWindow: UIWindow? {
        return
            connectedScenes
            .filter { $0.activationState == .foregroundActive }
            .compactMap { $0 as? UIWindowScene }
            .first?.windows
            .filter { $0.isKeyWindow }.first
    }
}

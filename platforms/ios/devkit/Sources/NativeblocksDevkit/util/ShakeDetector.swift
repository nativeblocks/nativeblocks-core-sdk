import Foundation
import Combine

#if os(iOS)
import UIKit

internal extension Notification.Name {
    static let deviceDidShake = Notification.Name("DeviceDidShakeNotification")
}

internal extension UIWindow {
    open override func motionEnded(_ motion: UIEvent.EventSubtype, with event: UIEvent?) {
        super.motionEnded(motion, with: event)

        if motion == .motionShake {
            NotificationCenter.default.post(name: .deviceDidShake, object: nil)
        }
    }
}

#endif

internal final class ShakeDetector: ObservableObject {

    public let onShakeDetected = PassthroughSubject<Void, Never>()
    private var isRunning = false

    public init() {
        setupShakeNotification()
    }

    public func start() {
        isRunning = true
    }

    public func stop() {
        isRunning = false
    }

    private func setupShakeNotification() {
        #if os(iOS)
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(handleShake),
            name: .deviceDidShake,
            object: nil
        )
        #endif
    }

    @objc private func handleShake() {
        guard isRunning else { return }
        onShakeDetected.send()
    }

    deinit {
        NotificationCenter.default.removeObserver(self)
    }
}

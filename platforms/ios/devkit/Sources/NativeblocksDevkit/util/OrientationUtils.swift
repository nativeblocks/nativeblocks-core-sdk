import Foundation
#if os(iOS)
    import UIKit
#endif

internal class OrientationUtils {
    private static let TAG = "OrientationChangeListener"
    enum OrientationType {
        case portrait
        case landscape
        case unknown
    }

    private var onOrientationDidChange: ((OrientationType) -> Void)? = nil

    func register(onOrientationDidChange: @escaping (OrientationType) -> Void) {
    #if os(iOS)
        self.onOrientationDidChange = onOrientationDidChange
        UIDevice.current.beginGeneratingDeviceOrientationNotifications()
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(orientationDidChange),
            name: UIDevice.orientationDidChangeNotification,
            object: nil
        )
    #endif
    }

    static func getOrientation() -> OrientationType {
    #if os(iOS)
        let orientation = UIDevice.current.orientation
        guard orientation != .unknown, orientation != .faceUp, orientation != .faceDown else {
            return .unknown
        }
        if orientation.isLandscape {
            DevKitLogger.debug(OrientationUtils.TAG, "getOrientation orientation:\(orientation), isLandscape:\(orientation.isLandscape)")
            return .landscape
        } else if orientation.isPortrait {
            DevKitLogger.debug(OrientationUtils.TAG, "getOrientation orientation:\(orientation), isPortrait:\(orientation.isPortrait)")
            return .portrait
        } else {
            return .unknown
        }
        
    #elseif os(macOS)
        return .unknown
    #endif
    }

    @objc private func orientationDidChange() {
        let orientation = OrientationUtils.getOrientation()
        DevKitLogger.debug(OrientationUtils.TAG, "orientationDidChange:\(orientation)")
        if orientation == .unknown {
            return
        } else {
            onOrientationDidChange?(orientation)
        }
    }

    func unregister() {
    #if os(iOS)
        if onOrientationDidChange != nil {
            NotificationCenter.default.removeObserver(self, name: UIDevice.orientationDidChangeNotification, object: nil)
            UIDevice.current.endGeneratingDeviceOrientationNotifications()
            onOrientationDidChange = nil
        }
    #endif
    }
}

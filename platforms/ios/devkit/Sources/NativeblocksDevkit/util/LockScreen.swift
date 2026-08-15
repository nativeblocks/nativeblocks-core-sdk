import Foundation
#if os(iOS)
    import UIKit
#endif


internal struct LockScreen {
    static func lockScreenOff() {
        #if os(iOS)
            UIApplication.shared.isIdleTimerDisabled = true
        #endif
    }

    static func lockScreenOn() {
        #if os(iOS)
            UIApplication.shared.isIdleTimerDisabled = false
        #endif
    }
}

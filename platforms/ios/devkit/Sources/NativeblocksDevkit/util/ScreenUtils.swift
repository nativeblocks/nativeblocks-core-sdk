import Foundation

#if canImport(UIKit)
    import UIKit
#endif
#if canImport(AppKit)
    import AppKit
#endif

internal struct ScreenUtils {
    #if canImport(UIKit)
        static func getScreenSize() -> CGSize {
            return UIScreen.main.bounds.size
        }
    #elseif canImport(AppKit)
        static func getScreenSize() -> CGSize? {
            guard let screen = NSScreen.main else {
                return nil
            }
            return screen.frame.size
        }
    #endif
}

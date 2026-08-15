import Foundation

#if os(iOS)
    import UIKit
#endif

internal struct DeviceInfo {
    static func getDeviceModelIdentifier() -> String {
        var systemInfo = utsname()
        uname(&systemInfo)
        let modelCode = withUnsafePointer(to: &systemInfo.machine) {
            $0.withMemoryRebound(to: CChar.self, capacity: 1) {
                String(validatingUTF8: $0)
            }
        }
        return modelCode ?? "Unknown"
    }

    static func getDeviceName() -> String {
        let modelIdentifier = getDeviceModelIdentifier()
        #if os(iOS)
            let name = UIDevice.current.name
            return modelIdentifier.hasPrefix(name) ? modelIdentifier : "\(name), \(modelIdentifier)"
        #elseif os(macOS)
            let name = Host.current().localizedName ?? modelIdentifier
            return modelIdentifier.hasPrefix(name) ? modelIdentifier : "\(name), \(modelIdentifier)"
        #else
            return "Unsupported Platform"
        #endif
    }
}

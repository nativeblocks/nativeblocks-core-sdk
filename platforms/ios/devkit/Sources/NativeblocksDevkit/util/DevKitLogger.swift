import Foundation

internal struct DevKitLogger {
    private static let isDebuggable = false
    static func debug(_ tag: String, _ message: String) {
        if DevKitLogger.isDebuggable {
            print("DevKitLogger \(tag): \(message)")
        }
    }

    static func error(_ tag: String, _ message: String, _ error: Error) {
        if DevKitLogger.isDebuggable {
            print("DevKitLogger \(tag): \(message)", error)
        }
    }

    static func error(_ tag: String, _ error: Error) {
        if DevKitLogger.isDebuggable {
            print("DevKitLogger \(tag):", error)
        }
    }
}

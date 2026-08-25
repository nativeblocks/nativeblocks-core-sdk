import Foundation

/// Lets UI tests open an instance and frame route directly instead of tapping
/// through the instance and frame lists, which would fold two network fetches
/// into whatever they are trying to measure.
struct LaunchArguments {

    static let instanceFlag = "-nbInstance"
    static let routeFlag = "-nbRoute"

    let instanceKey: String?
    let route: String?

    init(_ arguments: [String] = ProcessInfo.processInfo.arguments) {
        instanceKey = Self.value(of: Self.instanceFlag, in: arguments)
        route = Self.value(of: Self.routeFlag, in: arguments)
    }

    private static func value(of flag: String, in arguments: [String]) -> String? {
        guard let index = arguments.firstIndex(of: flag), index + 1 < arguments.count else {
            return nil
        }
        return arguments[index + 1]
    }

    func resolvedInstance(in instances: [InstanceInfo]) -> InstanceInfo? {
        return instances.first { $0.instanceKey == instanceKey } ?? instances.first
    }
}

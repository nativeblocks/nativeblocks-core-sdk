import Foundation
import NativeblocksRuntime

struct FrameRoute: Hashable {
    let name: String
    let route: String
}

extension NativeFrameRouteModel {

    func toFrameRoute() -> FrameRoute? {
        guard let route else { return nil }
        return FrameRoute(name: name ?? route, route: route)
    }
}

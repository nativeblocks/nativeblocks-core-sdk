import Foundation
import NativeblocksRuntimeFFI

internal typealias RuntimeFFIScaffoldModel = NativeblocksRuntimeFFI.ScaffoldModel
internal typealias RuntimeFFIFrameRouteModel = NativeblocksRuntimeFFI.FrameRouteModel
internal typealias RuntimeFFIRouteArgumentsModel = NativeblocksRuntimeFFI.RouteArgumentsModel
internal typealias RuntimeFFIFrameTypeModel = NativeblocksRuntimeFFI.FrameTypeModel

extension RuntimeFFIScaffoldModel {
    internal func toHost() -> NativeScaffoldModel {
        return NativeScaffoldModel(frames: frames.map { $0.toHost() })
    }
}

extension RuntimeFFIFrameRouteModel {
    fileprivate func toHost() -> NativeFrameRouteModel {
        return NativeFrameRouteModel(
            id: id,
            name: name,
            type: frameType?.toHost(),
            route: route,
            platform: platform,
            routeArguments: routeArguments?.map { $0.toHost() }
        )
    }
}

extension RuntimeFFIRouteArgumentsModel {
    fileprivate func toHost() -> NativeRouteArgumentsModel {
        return NativeRouteArgumentsModel(name: name)
    }
}

extension RuntimeFFIFrameTypeModel {
    fileprivate func toHost() -> FrameTypeModel {
        switch self {
        case .frame: return .frame
        case .bottomSheet: return .bottomSheet
        case .dialog: return .dialog
        }
    }
}

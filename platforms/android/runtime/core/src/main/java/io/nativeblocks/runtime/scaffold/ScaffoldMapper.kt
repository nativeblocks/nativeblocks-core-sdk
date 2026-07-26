package io.nativeblocks.runtime.scaffold

import io.nativeblocks.runtime.api.provider.model.FrameTypeModel
import io.nativeblocks.runtime.api.provider.model.NativeFrameRouteModel
import io.nativeblocks.runtime.api.provider.model.NativeRouteArgumentsModel
import io.nativeblocks.runtime.api.provider.model.NativeScaffoldModel
import io.nativeblocks.runtime.ffi.FrameRouteModel as RuntimeFFIFrameRouteModel
import io.nativeblocks.runtime.ffi.FrameTypeModel as RuntimeFFIFrameTypeModel
import io.nativeblocks.runtime.ffi.RouteArgumentsModel as RuntimeFFIRouteArgumentsModel
import io.nativeblocks.runtime.ffi.ScaffoldModel as RuntimeFFIScaffoldModel

internal fun RuntimeFFIScaffoldModel.toHost(): NativeScaffoldModel =
    NativeScaffoldModel(frames = frames.map { it.toHost() })

private fun RuntimeFFIFrameRouteModel.toHost(): NativeFrameRouteModel =
    NativeFrameRouteModel(
        id = id,
        name = name,
        type = frameType?.toHost(),
        route = route,
        platform = platform,
        routeArguments = routeArguments?.map { it.toHost() },
    )

private fun RuntimeFFIRouteArgumentsModel.toHost() =
    NativeRouteArgumentsModel(name = name)

private fun RuntimeFFIFrameTypeModel.toHost(): FrameTypeModel =
    when (this) {
        RuntimeFFIFrameTypeModel.FRAME -> FrameTypeModel.FRAME
        RuntimeFFIFrameTypeModel.BOTTOM_SHEET -> FrameTypeModel.BOTTOM_SHEET
        RuntimeFFIFrameTypeModel.DIALOG -> FrameTypeModel.DIALOG
    }

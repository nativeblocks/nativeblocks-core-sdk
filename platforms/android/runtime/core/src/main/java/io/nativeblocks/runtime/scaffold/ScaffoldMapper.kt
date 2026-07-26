package io.nativeblocks.runtime.scaffold

import io.nativeblocks.runtime.api.provider.model.FrameTypeModel
import io.nativeblocks.runtime.api.provider.model.NativeFrameRouteModel
import io.nativeblocks.runtime.api.provider.model.NativeRouteArgumentsModel
import io.nativeblocks.runtime.api.provider.model.NativeScaffoldModel
import io.nativeblocks.runtime.engine.FrameRouteModel as EngineFrameRouteModel
import io.nativeblocks.runtime.engine.FrameTypeModel as EngineFrameTypeModel
import io.nativeblocks.runtime.engine.RouteArgumentsModel as EngineRouteArgumentsModel
import io.nativeblocks.runtime.engine.ScaffoldModel as EngineScaffoldModel

internal fun EngineScaffoldModel.toHost(): NativeScaffoldModel =
    NativeScaffoldModel(frames = frames.map { it.toHost() })

private fun EngineFrameRouteModel.toHost(): NativeFrameRouteModel =
    NativeFrameRouteModel(
        id = id,
        name = name,
        type = frameType?.toHost(),
        route = route,
        platform = platform,
        routeArguments = routeArguments?.map { it.toHost() },
    )

private fun EngineRouteArgumentsModel.toHost() =
    NativeRouteArgumentsModel(name = name)

private fun EngineFrameTypeModel.toHost(): FrameTypeModel =
    when (this) {
        EngineFrameTypeModel.FRAME -> FrameTypeModel.FRAME
        EngineFrameTypeModel.BOTTOM_SHEET -> FrameTypeModel.BOTTOM_SHEET
        EngineFrameTypeModel.DIALOG -> FrameTypeModel.DIALOG
    }

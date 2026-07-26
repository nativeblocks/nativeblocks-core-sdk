package io.nativeblocks.sample.ui.frame

import io.nativeblocks.runtime.api.provider.model.NativeFrameRouteModel

data class FrameRoute(
    val name: String,
    val route: String,
)

internal fun NativeFrameRouteModel.toFrameRoute(): FrameRoute? {
    val route = route ?: return null
    return FrameRoute(name = name ?: route, route = route)
}

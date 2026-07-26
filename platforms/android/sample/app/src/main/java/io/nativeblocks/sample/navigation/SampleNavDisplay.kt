package io.nativeblocks.sample.navigation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.navigation3.runtime.NavBackStack
import androidx.navigation3.runtime.entryProvider
import androidx.navigation3.ui.NavDisplay
import io.nativeblocks.sample.instance.InstanceManager
import io.nativeblocks.sample.instance.instanceKey
import io.nativeblocks.sample.ui.frame.FrameListScreen
import io.nativeblocks.sample.ui.frame.FrameScreen
import io.nativeblocks.sample.ui.instance.InstanceListScreen

@Composable
fun SampleNavDisplay(
    instanceManager: InstanceManager,
    modifier: Modifier = Modifier,
) {
    val backStack = remember { NavBackStack<Destination>(Destination.Instances) }
    val goBack: () -> Unit = { backStack.removeLastOrNull() }

    NavDisplay(
        backStack = backStack,
        modifier = modifier,
        onBack = goBack,
        entryProvider = entryProvider {
            entry<Destination.Instances> {
                InstanceListScreen(
                    instanceManager = instanceManager,
                    onInstanceOpened = { instance ->
                        backStack.add(Destination.Frames(instance.instanceKey))
                    },
                )
            }
            entry<Destination.Frames> { key ->
                FrameListScreen(
                    instance = key.instance,
                    onFrameOpened = { frame ->
                        backStack.add(
                            Destination.Frame(
                                instance = key.instance,
                                route = frame.route,
                                title = frame.name,
                            )
                        )
                    },
                    onBack = goBack,
                )
            }
            entry<Destination.Frame> { key ->
                FrameScreen(
                    instance = key.instance,
                    route = key.route,
                    title = key.title,
                    onBack = goBack,
                )
            }
        },
    )
}

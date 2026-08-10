package io.nativeblocks.runtime.api

import androidx.compose.foundation.background
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import io.nativeblocks.runtime.di.NativeCoreSDKInjector
import io.nativeblocks.runtime.frame.FrameViewModel
import io.nativeblocks.runtime.frame.NativeFrame
import org.koin.androidx.compose.koinViewModel
import org.koin.compose.KoinIsolatedContext
import org.koin.core.qualifier.named

/**
 * Composable function for rendering a Nativeblocks frame.
 * Sets up the frame with the provided route and arguments, and handles loading and error states.
 *
 * @param route The route of the frame to display.
 * @param arguments A map of arguments to pass to the frame route.
 * @param state Whether the frame keeps what the user did to it, and under which key.
 * @param loading A composable to display during the loading state.
 * @param error A composable to display in case of an error, accepting an error message.
 */
@Composable
fun NativeblocksFrame(
    instanceName: String = "default",
    route: String,
    arguments: Map<String, String>,
    state: NativeblocksFrameState = NativeblocksFrameState.Stateless,
    loading: @Composable () -> Unit,
    error: @Composable (String) -> Unit
) {
    KoinIsolatedContext(context = NativeCoreSDKInjector.get(instanceName)) {
        val systemContractors = NativeblocksManager.getInstance(instanceName).providedActionContractors()
        systemContractors.forEach {
            it.ActionContractor()
        }
        val frameViewModel = koinViewModel<FrameViewModel>(
            qualifier = named(instanceName),
            key = "$instanceName-$route-${arguments.hashCode()}"
        )
        LaunchedEffect(instanceName, route, arguments, state) {
            frameViewModel.setupFrame(route, arguments, state.key)
        }
        NativeFrame(instanceName, frameViewModel, loading, error)
    }
}

/**
 * Composable function for displaying a loading indicator in Nativeblocks.
 * Adjusts the background and indicator color based on the system's dark mode setting.
 */
@Composable
fun NativeblocksLoading() {
    val bgColor = if (isSystemInDarkTheme()) Color.Gray else Color.White
    Column(
        Modifier
            .fillMaxSize()
            .background(bgColor),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Column {
            val color = if (isSystemInDarkTheme()) Color.White else Color.Black
            CircularProgressIndicator(color = color)
        }
    }
}

/**
 * Composable function for displaying an error message in Nativeblocks.
 * Adjusts the background and text color based on the system's dark mode setting.
 *
 * @param message The error message to display.
 */
@Composable
fun NativeblocksError(message: String) {
    val bgColor = if (isSystemInDarkTheme()) Color.Gray else Color.White
    Column(
        Modifier
            .fillMaxSize()
            .background(bgColor),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        val color = if (isSystemInDarkTheme()) Color.White else Color.Black
        Text(text = "Something went wrong!", color = color)
        Text(text = message, color = color)
    }
}
package io.nativeblocks.sample.ui.frame

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.nativeblocks.runtime.api.NativeblocksError
import io.nativeblocks.runtime.api.NativeblocksFrame
import io.nativeblocks.runtime.api.NativeblocksLoading

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FrameScreen(
    instance: String,
    route: String,
    title: String,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Scaffold(
        modifier = modifier,
        topBar = {
            TopAppBar(
                title = { Text(title) },
                navigationIcon = { TextButton(onClick = onBack) { Text("Back") } },
            )
        },
    ) { padding ->
        Box(Modifier.fillMaxSize().padding(padding)) {
            NativeblocksFrame(
                instance = instance,
                route = route,
                routeArguments = emptyMap(),
                loading = { NativeblocksLoading() },
                error = { message -> NativeblocksError(message) },
            )
        }
    }
}

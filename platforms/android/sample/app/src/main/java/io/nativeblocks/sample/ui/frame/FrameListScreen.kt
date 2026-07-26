package io.nativeblocks.sample.ui.frame

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import io.nativeblocks.runtime.api.NativeblocksManager

private sealed interface ScaffoldState {
    data object Loading : ScaffoldState
    data class Ready(val frames: List<FrameRoute>) : ScaffoldState
    data class Failed(val message: String) : ScaffoldState
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FrameListScreen(
    instance: String,
    onFrameOpened: (FrameRoute) -> Unit,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val state by produceState<ScaffoldState>(ScaffoldState.Loading, instance) {
        value = NativeblocksManager.getInstance(instance).getScaffold().fold(
            onSuccess = { scaffold ->
                ScaffoldState.Ready(scaffold.frames.mapNotNull { it.toFrameRoute() })
            },
            onFailure = { ScaffoldState.Failed(it.message ?: it.toString()) },
        )
    }

    Scaffold(
        modifier = modifier,
        topBar = {
            TopAppBar(
                title = { Text(instance) },
                navigationIcon = { TextButton(onClick = onBack) { Text("Back") } },
            )
        },
    ) { padding ->
        Box(Modifier.fillMaxSize().padding(padding)) {
            when (val current = state) {
                ScaffoldState.Loading ->
                    CircularProgressIndicator(Modifier.align(Alignment.Center))

                is ScaffoldState.Failed ->
                    Text(current.message, Modifier.align(Alignment.Center).padding(24.dp))

                is ScaffoldState.Ready ->
                    if (current.frames.isEmpty()) {
                        Text("No frames in this scaffold", Modifier.align(Alignment.Center))
                    } else {
                        LazyColumn(Modifier.fillMaxSize()) {
                            items(current.frames, key = { it.route }) { frame ->
                                ListItem(
                                    headlineContent = { Text(frame.name) },
                                    supportingContent = { Text(frame.route) },
                                    modifier = Modifier.clickable { onFrameOpened(frame) },
                                )
                                HorizontalDivider()
                            }
                        }
                    }
            }
        }
    }
}

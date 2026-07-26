package io.nativeblocks.sample.ui.instance

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import io.nativeblocks.sample.instance.InstanceInfo
import io.nativeblocks.sample.instance.InstanceManager
import io.nativeblocks.sample.instance.instanceKey

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun InstanceListScreen(
    instanceManager: InstanceManager,
    onInstanceOpened: (InstanceInfo) -> Unit,
    modifier: Modifier = Modifier,
) {
    Scaffold(
        modifier = modifier,
        topBar = { TopAppBar(title = { Text("Instances") }) },
    ) { padding ->
        LazyColumn(Modifier.fillMaxSize().padding(padding)) {
            items(instanceManager.instances, key = { it.instanceKey }) { instance ->
                ListItem(
                    headlineContent = { Text(instance.name) },
                    supportingContent = {
                        Text(if (instance.developmentMode) "development" else "production")
                    },
                    modifier = Modifier.clickable {
                        instanceManager.start(instance)
                        onInstanceOpened(instance)
                    },
                )
                HorizontalDivider()
            }
        }
    }
}

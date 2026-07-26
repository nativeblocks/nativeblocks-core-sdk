package io.nativeblocks.sample

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.ui.Modifier
import io.nativeblocks.sample.navigation.SampleNavDisplay
import io.nativeblocks.sample.ui.theme.SampleTheme

class MainActivity : ComponentActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val instanceManager = (application as SampleApplication).instanceManager
        setContent {
            SampleTheme {
                SampleNavDisplay(
                    instanceManager = instanceManager,
                    modifier = Modifier.fillMaxSize(),
                )
            }
        }
    }
}

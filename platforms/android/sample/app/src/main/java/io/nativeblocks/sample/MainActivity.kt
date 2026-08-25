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
    private val instanceManager by lazy { (application as SampleApplication).instanceManager }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            SampleTheme {
                SampleNavDisplay(
                    instanceManager = instanceManager,
                    modifier = Modifier.fillMaxSize(),
                )
            }
        }
    }

    override fun onDestroy() {
        instanceManager.stopAll()
        super.onDestroy()
    }
}

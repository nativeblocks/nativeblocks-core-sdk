package io.nativeblocks.runtime.api.provider.block.defaults

import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign

@Composable
internal fun InternalFallbackBlock(key: String) {
    Text(
        text = "The $key block isn’t available in this app version",
        color = Color.Gray,
        textAlign = TextAlign.Center
    )
}
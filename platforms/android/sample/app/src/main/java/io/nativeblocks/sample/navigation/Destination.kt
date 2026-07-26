package io.nativeblocks.sample.navigation

import androidx.navigation3.runtime.NavKey
sealed interface Destination : NavKey {

    data object Instances : Destination
    data class Frames(val instance: String) : Destination
    data class Frame(
        val instance: String,
        val route: String,
        val title: String,
    ) : Destination
}

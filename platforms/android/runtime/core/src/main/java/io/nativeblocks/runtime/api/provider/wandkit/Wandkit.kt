package io.nativeblocks.runtime.api.provider.wandkit

import android.content.Context
import io.nativeblocks.runtime.api.NativeblocksEdition

/**
 * Defines the contract for managing the WandKit functionality within the native framework.
 */
interface Wandkit {

    /**
     * Sets up the WandKit with the specified context and edition.
     * @param context The Android context required for initialization.
     * @param edition The edition of Nativeblocks being used.
     * @param instanceName The name of Nativeblocks instance being used.
     */
    fun setup(context: Context, edition: NativeblocksEdition, instanceName: String)

    /**
     * Cleans up and releases any resources used by the WandKit.
     */
    fun destroy()
}
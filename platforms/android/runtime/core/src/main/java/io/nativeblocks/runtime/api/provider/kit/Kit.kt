package io.nativeblocks.runtime.api.provider.kit

import android.content.Context
import io.nativeblocks.runtime.api.NativeblocksEdition

interface Kit {

    fun attach(
        context: Context,
        instanceName: String,
        edition: NativeblocksEdition
    )

    fun detach(instanceName: String)
}

package io.nativeblocks.sample

import android.app.Application
import io.nativeblocks.sample.instance.InstanceManager

class SampleApplication : Application() {

    val instanceManager: InstanceManager by lazy { InstanceManager(this) }
}

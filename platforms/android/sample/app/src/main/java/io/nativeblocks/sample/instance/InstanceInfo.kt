package io.nativeblocks.sample.instance

/**
 * One Nativeblocks project the sample can talk to.
 *
 * Each entry becomes an independently initialised SDK instance, which is what
 * makes the sample multi-instance: several projects can be live at once without
 * sharing cache, frames or providers.
 */
data class InstanceInfo(
    val name: String,
    val apiKey: String,
    val apiUrl: String,
    val developmentMode: Boolean = false,
)

/**
 * The name `NativeblocksManager` is keyed by.
 *
 * Two entries may share a [name] while differing in [developmentMode] — the same
 * project in production and in development. The SDK keys instances by name
 * alone, so the mode is folded in here; without it the second entry would
 * silently resolve to the first one's manager.
 */
val InstanceInfo.instanceKey: String
    get() = if (developmentMode) "$name-dev" else name

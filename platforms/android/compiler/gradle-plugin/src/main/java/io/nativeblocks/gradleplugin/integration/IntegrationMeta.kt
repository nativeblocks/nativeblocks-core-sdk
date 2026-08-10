package io.nativeblocks.gradleplugin.integration

import kotlinx.serialization.json.JsonElement

internal data class IntegrationMeta(
    val integrationJson: JsonElement?,
    val propertiesJson: JsonElement?,
    val eventsJson: JsonElement?,
    val dataJson: JsonElement?,
    val slotsJson: JsonElement?,
)
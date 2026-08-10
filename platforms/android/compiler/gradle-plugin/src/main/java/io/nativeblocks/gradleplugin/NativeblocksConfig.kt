package io.nativeblocks.gradleplugin

import kotlinx.serialization.Serializable
import java.io.Serializable as JavaSerializable

@Serializable
data class NativeblocksConfig(
    val endpoint: String = "",
    val authToken: String = "",
    val organizationId: String = ""
) : JavaSerializable
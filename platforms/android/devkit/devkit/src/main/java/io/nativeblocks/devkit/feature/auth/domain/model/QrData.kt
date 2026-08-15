package io.nativeblocks.devkit.feature.auth.domain.model

import kotlinx.serialization.Serializable

@Serializable
internal data class QrData(
    val endpoint: String,
    val token: String,
    val realtimeEndpoint: String,
)
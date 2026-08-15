package io.nativeblocks.devkit.feature.preview.domain.model

import java.util.UUID

internal data class Parameter(
    val id: String = UUID.randomUUID().toString(),
    val key: String,
    val value: String
)

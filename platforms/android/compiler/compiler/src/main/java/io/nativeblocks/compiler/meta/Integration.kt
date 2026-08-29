package io.nativeblocks.compiler.meta

import kotlinx.serialization.Serializable
import kotlinx.serialization.Transient

@Serializable
internal data class Integration(
    val name: String,
    val scope: String?,
    val description: String,
    val version: Int,
    val versionName: String,
    val deprecated: Boolean,
    val deprecatedReason: String,
    val documentation: String,
    val imageIcon: String,
    val keyType: String,
    val kind: String,
    val platformSupport: String,
    val price: Int,
    val public: Boolean,
)

@Serializable
internal data class Data(
    val key: String,
    val type: String,
    val description: String,
    val deprecated: Boolean,
    val deprecatedReason: String,
    @Transient val value: String = "",
    @Transient val typeClass: TypeClass = TypeClass()
)

@Serializable
internal data class Property(
    val key: String,
    val value: String,
    val type: String,
    val description: String,
    val deprecated: Boolean,
    val deprecatedReason: String,
    val valuePicker: String,
    val valuePickerGroup: String,
    val valuePickerOptions: String,
    @Transient val typeClass: TypeClass = TypeClass()
)

@Serializable
internal data class TypeClass(
    val packageName: String = "",
    val simpleNames: List<String> = emptyList()
) {
    val canonicalName: String
        get() = (if (packageName.isEmpty()) simpleNames else listOf(packageName) + simpleNames).joinToString(".")
}

@Serializable
internal data class ValuePickerOption(
    val id: String,
    val text: String
)

@Serializable
internal data class Event(
    val event: String,
    val scope: String?,
    val description: String,
    val deprecated: Boolean,
    val deprecatedReason: String,
    val dataBindings: List<String> = listOf(),
    @Transient val functionName: String = "",
)

@Serializable
internal data class Slot(
    val slot: String,
    val scope: String?,
    val description: String,
    val deprecated: Boolean,
    val deprecatedReason: String,
    val dataBindings: List<String> = listOf(),
)

@Serializable
internal data class ExtraParam(
    val key: String,
    val type: String,
)
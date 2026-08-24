package io.nativeblocks.compiler.util

internal val PRIMITIVE_TYPES = setOf(
    "kotlin.String",
    "kotlin.Int",
    "kotlin.Long",
    "kotlin.Float",
    "kotlin.Double",
    "kotlin.Boolean",
)

internal fun converterVar(canonicalName: String): String =
    "converter" + canonicalName.replace(Regex("[^A-Za-z0-9]"), "_")

internal fun valueConversion(canonicalName: String, source: String, default: String): String {
    return when (canonicalName) {
        "kotlin.String" -> """$source ?: "${default.stringify()}""""
        "kotlin.Int" -> """$source?.toIntOrNull() ?: ${default.ifEmpty { 0 }}"""
        "kotlin.Long" -> """$source?.toLongOrNull() ?: (${default.ifEmpty { "0" }}).toLong()"""
        "kotlin.Float" -> """$source?.toFloatOrNull() ?: (${default.ifEmpty { "0.0" }}).toFloat()"""
        "kotlin.Double" -> """$source?.toDoubleOrNull() ?: (${default.ifEmpty { "0.0" }}).toDouble()"""
        "kotlin.Boolean" -> """$source?.lowercase()?.toBooleanStrictOrNull() ?: ${default.ifEmpty { false }}"""
        else -> """${converterVar(canonicalName)}.fromString($source ?: "${default.stringify()}")"""
    }
}

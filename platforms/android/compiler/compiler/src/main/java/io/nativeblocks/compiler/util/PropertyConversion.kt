package io.nativeblocks.compiler.util

internal val PRIMITIVE_PROP_TYPES = setOf(
    "kotlin.String",
    "kotlin.Int",
    "kotlin.Long",
    "kotlin.Float",
    "kotlin.Double",
    "kotlin.Boolean",
)

internal fun converterVar(canonicalName: String): String =
    "converter" + canonicalName.replace(Regex("[^A-Za-z0-9]"), "_")

internal fun propertyConversion(canonicalName: String, source: String, default: String): String {
    return when (canonicalName) {
        "kotlin.String" -> """$source ?: "${default.stringify()}""""
        "kotlin.Int" -> """$source?.toIntOrNull() ?: ${default.ifEmpty { 0 }}"""
        "kotlin.Long" -> """$source?.toLongOrNull() ?: ${default.ifEmpty { 0L }}"""
        "kotlin.Float" -> """$source?.toFloatOrNull() ?: ${default.ifEmpty { 0.0F }}"""
        "kotlin.Double" -> """$source?.toDoubleOrNull() ?: ${default.ifEmpty { 0.0 }}"""
        "kotlin.Boolean" -> """$source?.lowercase()?.toBooleanStrictOrNull() ?: ${default.ifEmpty { false }}"""
        else -> """${converterVar(canonicalName)}.fromString($source ?: "${default.stringify()}")"""
    }
}

internal fun dataConversion(type: String, source: String, default: String, key: String): String {
    return when (type) {
        "STRING" -> """$source ?: "${default.stringify()}""""
        "INT" -> """$source?.toIntOrNull() ?: ${default.ifEmpty { 0 }}"""
        "LONG" -> """$source?.toLongOrNull() ?: ${default.ifEmpty { 0L }}"""
        "FLOAT" -> """$source?.toFloatOrNull() ?: ${default.ifEmpty { 0.0F }}"""
        "DOUBLE" -> """$source?.toDoubleOrNull() ?: ${default.ifEmpty { 0.0 }}"""
        "BOOLEAN" -> """$source?.lowercase()?.toBooleanStrictOrNull() ?: ${default.ifEmpty { false }}"""
        else -> throw Diagnostic.exceptionDispatcher(DiagnosticType.MetaCustomType(key, type))
    }
}

package io.nativeblocks.compiler.util

import com.google.devtools.ksp.symbol.KSAnnotated
import com.google.devtools.ksp.symbol.KSAnnotation
import java.io.OutputStream
import java.util.Locale

internal operator fun OutputStream.plusAssign(string: String) {
    write(string.toByteArray())
}

internal fun KSAnnotated.getAnnotation(predicate: (KSAnnotation) -> Boolean) =
    annotations.first { predicate(it) }

internal fun KSAnnotated.getAnnotation(fullName: String) = getAnnotation {
    it.shortName.asString() == fullName
}

internal inline fun <reified T> KSAnnotation.getArgument(name: String) =
    arguments.first { it.name?.asString() == name }.value as T

internal fun String.capitalize() =
    replaceFirstChar { if (it.isLowerCase()) it.titlecase(Locale.ROOT) else it.toString() }

internal fun String.camelcase() =
    replaceFirstChar { if (it.isUpperCase()) it.lowercase() else it.toString() }

fun String.onlyLettersAndUnderscore() = all { it.isLetter() || it == '_' }

internal fun String.stringify() = this.replace("\\", "\\\\").replace("\"", "\\\"")
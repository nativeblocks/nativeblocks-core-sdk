package io.nativeblocks.runtime.util

internal fun String.isValidInstanceName(): Boolean {
    val allowedPattern = "^[A-Za-z0-9_-]+$".toRegex()
    return this.isNotBlank() && allowedPattern.matches(this)
}
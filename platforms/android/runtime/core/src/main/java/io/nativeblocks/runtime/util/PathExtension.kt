package io.nativeblocks.runtime.util

import io.nativeblocks.runtime.ffi.isValidInstanceName as ffiIsValidInstanceName

internal fun String.isValidInstanceName(): Boolean = ffiIsValidInstanceName(this)

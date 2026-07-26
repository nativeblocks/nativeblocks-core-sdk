package io.nativeblocks.runtime.lib

import io.nativeblocks.runtime.ffi.ErrorType
import io.nativeblocks.runtime.ffi.NbException

internal inline fun <R> uniffiCallbackGuard(errorType: ErrorType, block: () -> R): R = try {
    block()
} catch (e: NbException) {
    throw e
} catch (e: Throwable) {
    throw NbException.Failure(
        reason = e.message ?: e.toString(),
        errorType = errorType,
        errorCode = null,
    )
}

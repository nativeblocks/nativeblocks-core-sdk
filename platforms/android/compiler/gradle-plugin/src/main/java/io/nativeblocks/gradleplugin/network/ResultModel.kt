package io.nativeblocks.gradleplugin.network

internal sealed class ResultModel<out V> {

    data class Success<V>(val value: V) : ResultModel<V>()

    data class Error(val error: ErrorModel) : ResultModel<Nothing>()
}

internal data class ErrorModel(
    val message: String? = null,
    val errorType: String? = null,
)

internal fun <R> ResultModel<R>.doOnSuccess(ifSuccess: (value: R) -> Unit) {
    if (this is ResultModel.Success) {
        ifSuccess.invoke(this.value)
    }
}

internal fun <R> ResultModel<R>.doOnError(ifError: (value: ErrorModel) -> Unit) {
    if (this is ResultModel.Error) {
        ifError.invoke(this.error)
    }
}

internal fun <T, R> ResultModel<T>.map(isSuccess: (T) -> R): ResultModel<R> {
    return when (this) {
        is ResultModel.Success -> ResultModel.Success(isSuccess.invoke(value))
        is ResultModel.Error -> ResultModel.Error(error)
    }
}

internal inline fun <R, T> ResultModel<T>.execute(
    ifSuccess: (value: T) -> R,
    ifError: (error: ErrorModel) -> R
): R {
    return when (this) {
        is ResultModel.Success<T> -> ifSuccess(value)
        is ResultModel.Error -> ifError(ErrorModel(error.message))
    }
}

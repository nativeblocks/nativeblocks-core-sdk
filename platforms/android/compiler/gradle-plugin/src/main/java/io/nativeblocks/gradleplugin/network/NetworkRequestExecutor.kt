package io.nativeblocks.gradleplugin.network

import com.apollographql.apollo3.ApolloCall
import com.apollographql.apollo3.api.ApolloResponse
import com.apollographql.apollo3.api.Operation

internal class NetworkRequestExecutor {
    suspend fun <D : Operation.Data> requestExecutor(result: ApolloCall<D>): ResultModel<D?> {
        return try {
            val response = result.execute()
            when {
                response.hasErrors() -> errorHandler(response)
                response.data != null -> ResultModel.Success(response.data)
                else -> errorHandler(response)
            }
        } catch (e: Exception) {
            ResultModel.Error(
                ErrorModel(
                    e.message ?: "Please try again",
                    GraphqlErrorTypes.UNKNOWN_ERROR.name
                )
            )
        }
    }

    private fun <Q : Operation.Data> errorHandler(response: ApolloResponse<Q>): ResultModel.Error {
        val errorType = response.errors?.first()?.extensions?.get("classification").toString()
        return ResultModel.Error(
            ErrorModel(message = response.errors?.first()?.message, errorType = errorType)
        )
    }
}

internal enum class GraphqlErrorTypes {
    UNKNOWN_ERROR, BAD_REQUEST, UNAUTHORIZED, UNAUTHORIZED_SDK, NOT_FOUND, INTERNAL_ERROR
}
package io.nativeblocks.runtime.api.provider.model

/**
 * A template resolver to resolve formats for the block
 */
interface TemplateResolver {
    fun resolve(value: String?): String?
}

internal fun resolveIn(scope: Any?, value: String?): String?  {
    return (scope as? TemplateResolver)?.resolve(value) ?: value
}

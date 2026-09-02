package io.nativeblocks.foundation.util

import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.runtime.Composable
import io.nativeblocks.runtime.api.provider.model.TemplateResolver
import org.json.JSONObject

/**
 * The scope a list hands to the blocks that say what it contains.
 *
 * It stands where the list itself stands, so it answers for the row the list sits in:
 * a list inside a row reads that row's values to find what to repeat over.
 */
class LazyListDescribeScope(
    private val scope: LazyListScope,
    private val memory: MutableMap<Any, Pair<Any?, Any?>>,
    private val parentScope: Any? = null,
) : TemplateResolver {

    override fun resolve(value: String?): String? =
        (parentScope as? TemplateResolver)?.resolve(value) ?: value

    fun item(key: Any?, content: @Composable (scope: Any) -> Unit) {
        scope.item(key = key) { content(this) }
    }

    fun items(
        count: Int,
        key: (index: Int) -> Any,
        root: String,
        element: (index: Int) -> JSONObject?,
        content: @Composable (index: Int, scope: Any) -> Unit,
    ) {
        scope.items(count = count, key = key) { index ->
            content(index, ListRowScope(this, root, element(index), parentScope))
        }
    }

    @Suppress("UNCHECKED_CAST")
    fun <T> cached(id: Any, on: Any?, compute: () -> T): T {
        val existing = memory[id]
        if (existing != null && existing.first == on) return existing.second as T
        val computed = compute()
        memory[id] = on to computed
        return computed
    }
}

package io.nativeblocks.foundation.util

import io.nativeblocks.runtime.api.provider.model.TemplateResolver
import org.json.JSONArray
import org.json.JSONObject

data class ListItemModel(val id: String, val value: JSONObject)

fun readListItems(listJson: String?, idPath: String?): List<ListItemModel> {
    val elements = splitArray(listJson)
    if (idPath.isNullOrEmpty()) return byPosition(elements)

    val ids = mutableListOf<String>()
    for (element in elements) {
        val id = readPath(element, idPath) ?: return byPosition(elements)
        if (ids.contains(id)) return byPosition(elements)
        ids.add(id)
    }
    return ids.zip(elements) { id, value -> ListItemModel(id, value) }
}

internal fun readPath(value: JSONObject, path: String): String? {
    var current: Any? = value
    for (segment in path.split('.')) {
        current = (current as? JSONObject)?.opt(segment) ?: return null
    }
    return when (current) {
        JSONObject.NULL -> null
        is String -> current as String
        else -> current?.toString()
    }
}

private fun byPosition(elements: List<JSONObject>) =
    elements.mapIndexed { index, value -> ListItemModel(index.toString(), value) }

private fun splitArray(json: String?): List<JSONObject> {
    val text = json?.trim().orEmpty()
    if (text.isEmpty()) return emptyList()
    val array = runCatching { JSONArray(text) }.getOrNull() ?: return emptyList()
    return (0 until array.length()).mapNotNull { index -> array.optJSONObject(index) }
}

/**
 * {{json:<root>.<path>}} — the root names the block that reports the row.
 *
 * Every brace is escaped: the device runs this through ICU, which rejects a bare
 * closing brace that the JVM quietly accepts.
 */
internal val JSON_REF = Regex("""\{\{json:([A-Za-z0-9_-]+)\.([^}]+)\}\}""")

/**
 * The scope a row is built in: the element it belongs to, under the key of the block
 * that reports it, plus the scope that block itself sat in.
 *
 * A reference names its root, so a list inside a list resolves against the row that
 * actually reports that root rather than the nearest one.
 */
class ListRowScope(
    val itemScope: Any,
    private val root: String,
    private val element: JSONObject?,
    private val parent: Any?,
) : TemplateResolver {

    override fun resolve(value: String?): String? {
        if (value == null || !value.contains("{{json:")) return value
        return JSON_REF.replace(value) { match ->
            val (root, path) = match.destructured
            read(root, path) ?: match.value
        }
    }

    private fun read(root: String, path: String): String? {
        if (root == this.root) {
            return element?.let { readPath(it, path) }
        }
        return (parent as? ListRowScope)?.read(root, path)
    }
}

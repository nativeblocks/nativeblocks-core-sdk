import Foundation
import NativeblocksRuntime

public struct ListItemModel: Identifiable {
    public let id: String
    public let value: [String: Any]
}

public func readListItems(_ listJson: String?, _ idPath: String?) -> [ListItemModel] {
    let elements = splitArray(listJson)
    guard let idPath, !idPath.isEmpty else { return byPosition(elements) }

    var ids: [String] = []
    for element in elements {
        guard let id = readPath(element, idPath), !ids.contains(id) else {
            return byPosition(elements)
        }
        ids.append(id)
    }
    return zip(ids, elements).map { ListItemModel(id: $0, value: $1) }
}

func readPath(_ value: [String: Any], _ path: String) -> String? {
    var current: Any? = value
    for segment in path.split(separator: ".") {
        guard let object = current as? [String: Any] else { return nil }
        current = object[String(segment)]
    }
    switch current {
    case nil, is NSNull:
        return nil
    case let text as String:
        return text
    case let number as NSNumber:
        if CFGetTypeID(number) == CFBooleanGetTypeID() { return number.boolValue ? "true" : "false" }
        return number.stringValue
    default:
        // An array or object is read back as JSON text, so a path can name the list a
        // nested items block repeats over.
        guard let current, let data = try? JSONSerialization.data(withJSONObject: current) else { return nil }
        return String(data: data, encoding: .utf8)
    }
}

private func byPosition(_ elements: [[String: Any]]) -> [ListItemModel] {
    return elements.enumerated().map { ListItemModel(id: String($0.offset), value: $0.element) }
}

private func splitArray(_ json: String?) -> [[String: Any]] {
    guard let text = json?.trimmingCharacters(in: .whitespacesAndNewlines), !text.isEmpty,
        let parsed = try? JSONSerialization.jsonObject(with: Data(text.utf8)) as? [Any]
    else { return [] }
    return parsed.compactMap { $0 as? [String: Any] }
}

/// `{{json:<root>.<path>}}` — the root names the block that reports the row.
private let jsonRef = try? NSRegularExpression(pattern: "\\{\\{json:([A-Za-z0-9_-]+)\\.([^}]+)\\}\\}")

/// The scope a row is built in: the element it belongs to, under the key of the block
/// that reports it, plus the scope that block itself sat in.
///
/// A reference names its root, so a list inside a list resolves against the row that
/// actually reports that root rather than the nearest one.
public struct ListRowScope: TemplateResolver {
    private let root: String
    private let element: [String: Any]
    private let parent: Any?

    public init(root: String, element: [String: Any], parent: Any?) {
        self.root = root
        self.element = element
        self.parent = parent
    }

    public func resolve(_ value: String?) -> String? {
        guard let value, value.contains("{{json:"), let pattern = jsonRef else { return value }
        let text = value as NSString
        var result = ""
        var cursor = 0
        for match in pattern.matches(in: value, range: NSRange(location: 0, length: text.length)) {
            result += text.substring(with: NSRange(location: cursor, length: match.range.location - cursor))
            let root = text.substring(with: match.range(at: 1))
            let path = text.substring(with: match.range(at: 2))
            result += read(root, path) ?? text.substring(with: match.range)
            cursor = match.range.location + match.range.length
        }
        result += text.substring(from: cursor)
        return result
    }

    private func read(_ root: String, _ path: String) -> String? {
        if root == self.root { return readPath(element, path) }
        return (parent as? ListRowScope)?.read(root, path)
    }
}

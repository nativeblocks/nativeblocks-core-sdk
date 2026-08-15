import Foundation

internal protocol ParameterRepository {
    func getAll() -> [PreviewParameter]
    func add(key: String, value: String) -> PreviewParameter
    func update(id: String, key: String, value: String)
    func delete(id: String)
    func clear()
    func toMap() -> [String: String]
}

import Combine

internal protocol INativeCache {
    func push(key: String, value: String?)

    func pull(key: String, defaultValue: String) -> String

    func clear()

    func delete(key: String)

    func onValueChange(key: String, defaultValue: String) -> AnyPublisher<String, Never>
}

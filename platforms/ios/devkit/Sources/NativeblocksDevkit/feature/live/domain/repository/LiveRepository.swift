import Combine

internal protocol LiveRepository {
    func start()
    func end()
    func orientedChange()
    func connectionState() -> AnyPublisher<ConnectionState, Never>
    func hotReload() -> AnyPublisher<String, Never>
    func cleanUp()
}

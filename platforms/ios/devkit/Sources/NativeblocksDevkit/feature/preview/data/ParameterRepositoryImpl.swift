import Foundation

internal final class ParameterRepositoryImpl: ParameterRepository {

    private let storageKey: String
    private var parameters: [PreviewParameter] = []

    init(instanceName: String) {
        self.storageKey = "PreviewKit_Parameters_\(instanceName)"
        loadFromStorage()
    }

    func getAll() -> [PreviewParameter] {
        return parameters
    }

    @discardableResult
    func add(key: String, value: String) -> PreviewParameter {
        let parameter = PreviewParameter(
            id: UUID().uuidString,
            name: key,
            value: value
        )
        parameters.append(parameter)
        saveToStorage()
        return parameter
    }

    func update(id: String, key: String, value: String) {
        if let index = parameters.firstIndex(where: { $0.id == id }) {
            parameters[index] = PreviewParameter(id: id, name: key, value: value)
            saveToStorage()
        }
    }

    func delete(id: String) {
        parameters.removeAll { $0.id == id }
        saveToStorage()
    }

    func clear() {
        parameters.removeAll()
        saveToStorage()
    }

    func toMap() -> [String: String] {
        var result: [String: String] = [:]
        for param in parameters {
            result[param.name] = param.value
        }
        return result
    }

    private func loadFromStorage() {
        guard let data = UserDefaults.standard.data(forKey: storageKey),
              let stored = try? JSONDecoder().decode([StoredParameter].self, from: data) else {
            parameters = []
            return
        }

        parameters = stored.map { stored in
            PreviewParameter(id: stored.id, name: stored.key, value: stored.value)
        }
    }

    private func saveToStorage() {
        let stored = parameters.map { param in
            StoredParameter(id: param.id, key: param.name, value: param.value)
        }

        if let data = try? JSONEncoder().encode(stored) {
            UserDefaults.standard.set(data, forKey: storageKey)
        }
    }
}

private struct StoredParameter: Codable {
    let id: String
    let key: String
    let value: String
}

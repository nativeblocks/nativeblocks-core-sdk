import Combine
import Foundation

internal class NativeCacheStorage: INativeCache {
    private let userDefaults: UserDefaults
    private let prefix: String

    private var subjects: [String: PassthroughSubject<String, Never>] = [:]

    init(userDefaults: UserDefaults, prefix: String = "") {
        self.userDefaults = userDefaults
        self.prefix = prefix
    }

    func push(key: String, value: String?) {
        let value = value ?? ""
        userDefaults.set(value, forKey: prefix + key)
        subject(for: key).send(value)
    }

    func pull(key: String, defaultValue: String) -> String {
        return userDefaults.string(forKey: prefix + key) ?? defaultValue
    }

    func clear() {
        for key in subjects.keys {
            delete(key: key)
        }
    }

    func delete(key: String) {
        userDefaults.removeObject(forKey: prefix + key)
        subject(for: key).send("")
    }

    func onValueChange(key: String, defaultValue: String) -> AnyPublisher<String, Never> {
        return subject(for: key).eraseToAnyPublisher()
    }

    private func subject(for key: String) -> PassthroughSubject<String, Never> {
        if let subject = subjects[key] {
            return subject
        }
        let subject = PassthroughSubject<String, Never>()
        subjects[key] = subject
        return subject
    }
}

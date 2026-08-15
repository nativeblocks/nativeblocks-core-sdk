import Foundation

internal struct PreviewParameter: Identifiable {
    public let id: String
    public let name: String
    public var value: String

    public init(id: String, name: String, value: String) {
        self.id = id
        self.name = name
        self.value = value
    }
}

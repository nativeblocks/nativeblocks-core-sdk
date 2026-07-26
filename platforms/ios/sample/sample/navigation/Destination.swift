import Foundation

enum Destination: Hashable {
    case frames(instance: String)
    case frame(instance: String, route: String, title: String)
}

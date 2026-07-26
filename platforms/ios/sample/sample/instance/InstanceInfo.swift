import Foundation

struct InstanceInfo: Hashable {
    let name: String
    let apiKey: String
    let apiUrl: String
    var developmentMode: Bool = false
}

extension InstanceInfo {

    var instanceKey: String {
        developmentMode ? "\(name)-dev" : name
    }
}

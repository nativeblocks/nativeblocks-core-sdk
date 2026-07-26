import Foundation

enum ArgumentError: Error, CustomStringConvertible {
    case missingTarget
    case missingDirectory
    case missingEndpoint
    case missingAuthToken
    case missingOrganizationId
    case missingCommand
    case invalidDirectory(String)
    case extraArguments([String])

    var localizedDescription: String { return description }

    var description: String {
        switch self {
        case .missingTarget:
            return "--target argument is missing."
        case .missingDirectory:
            return "--directory argument is missing."
        case .missingEndpoint:
            return "--endpoint argument is missing."
        case .missingAuthToken:
            return "--authToken argument is missing."
        case .missingOrganizationId:
            return "--organizationId argument is missing."
        case .missingCommand:
            return "please use commands like: 'generate-provider', 'sync', 'prepare-schema'"
        case .invalidDirectory(let path):
            return "The directory \(path) does not exist."
        case .extraArguments(let extraArgs):
            return "Extra arguments provided: \(extraArgs.joined(separator: ", "))"
        }
    }
}

import Foundation

enum ArgumentError: Error, CustomStringConvertible {
    case missingTarget
    case missingDirectory
    case missingOutput
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
        case .missingOutput:
            return "--output argument is missing."
        case .missingCommand:
            return "please use commands like: 'generate-provider', 'prepare-schema'"
        case .invalidDirectory(let path):
            return "The directory \(path) does not exist."
        case .extraArguments(let extraArgs):
            return "Extra arguments provided: \(extraArgs.joined(separator: ", "))"
        }
    }
}

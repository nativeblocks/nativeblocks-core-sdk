import Foundation

/// A template resolver to resolve formats for the block
public protocol TemplateResolver {
    func resolve(_ value: String?) -> String?
}

func resolveIn(_ scope: Any?, _ value: String?) -> String? {
    return (scope as? TemplateResolver)?.resolve(value) ?? value
}

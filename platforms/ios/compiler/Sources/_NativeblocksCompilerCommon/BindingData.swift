import SwiftSyntax

public func generateBindingDataJson(for event: EventMeta, startPosition: Int) -> [BindingDataMeta] {
    guard let binding = event.variable else { return [] }
    let parameters = Array(SyntaxUtils.extractFunctionType(from: binding)?.parameters ?? [])
    var position = startPosition

    return event.dataBindings.enumerated().compactMap { index, key in
        guard parameters.indices.contains(index) else { return nil }
        position += 1
        return BindingDataMeta(
            data: DataMeta(
                position: position,
                key: key,
                type: parameters[index].type.trimmedDescription,
                description: "",
                deprecated: false,
                deprecatedReason: "",
                block: event.block,
                variable: binding,
                value: ""
            )
        )
    }
}

public func undeclaredBindingData(declared: [DataMeta], bindings: [BindingDataMeta]) -> [BindingDataMeta] {
    var kept: [BindingDataMeta] = []
    for candidate in bindings {
        if declared.contains(where: { $0.key == candidate.data.key }) { continue }
        if kept.contains(where: { $0.data.key == candidate.data.key }) { continue }
        kept.append(candidate)
    }
    return kept
}

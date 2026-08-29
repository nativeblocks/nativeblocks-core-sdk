import SwiftDiagnostics
import SwiftSyntax
import SwiftSyntaxBuilder

public enum ModifierExtractor {
    static let NativeModifierDataType = "ModifierData"
    static let NativeModifierEventType = "ModifierEvent"

    public static func extractVariable(from structDecl: StructDeclSyntax) -> ([NativeMeta], [Diagnostic]) {
        var meta: [NativeMeta] = []
        var errors: [Diagnostic] = []
        var position = 0
        for member in structDecl.memberBlock.members {
            guard let varDecl = member.decl.as(VariableDeclSyntax.self) else { continue }
            position += 1

            let (type, _) = SyntaxUtils.getType(from: varDecl) ?? (nil, nil)

            switch type {
            case NativeModifierDataType:
                guard let (block, blockErrors) = extractDataModifier(from: varDecl, startPosition: position) else {
                    continue
                }
                position = block.last?.position ?? position
                errors.append(contentsOf: blockErrors)
                meta.append(contentsOf: block)
            case NativeModifierEventType:
                guard let (block, blockErrors) = extractEventModifier(from: varDecl, startPosition: position) else {
                    continue
                }
                position = block.last?.position ?? position
                errors.append(contentsOf: blockErrors)
                meta.append(contentsOf: block)
            default:
                guard let (block, blockErrors) = extractExtraParam(from: varDecl, startPosition: position) else {
                    continue
                }
                for param in block {
                    if param.key == "modifierContext" && param.type == "ModifierContext" {
                        meta.append(param)
                    }
                }
                position = block.last?.position ?? position
                errors.append(contentsOf: blockErrors)
            }
        }

        let dataModifiers = meta.compactMap { $0 as? DataMeta }
        let eventModifiers = meta.compactMap { $0 as? EventMeta }

        let startPosition = max(dataModifiers.map { $0.position }.max() ?? 0, eventModifiers.map { $0.position }.max() ?? 0)
        let bindingData = eventModifiers.flatMap { generateBindingDataJson(for: $0, startPosition: startPosition) }
        meta.append(contentsOf: undeclaredBindingData(declared: dataModifiers, bindings: bindingData))
        return (meta, errors)
    }

    private static func extractDataModifier(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([DataMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var diagnostic: [Diagnostic] = []

        guard let blockAttribute = SyntaxUtils.extractAttribute(for: NativeModifierDataType, from: attributes) else {
            return nil
        }

        if attributes.count > 1 {
            diagnostic.append(Diagnostic(node: blockAttribute, message: DiagnosticType.multiAttributes))
        }

        let description = SyntaxUtils.extractDescription(from: blockAttribute) ?? ""
        let deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute) ?? false
        let deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute) ?? ""
        let defaultValue = SyntaxUtils.extractDefaultValue(from: blockAttribute) ?? ""

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let key = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""
                let type = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(IdentifierTypeSyntax.self)?.name.text ?? ""

                return !key.isEmpty && !type.isEmpty
                    ? DataMeta(
                        position: position,
                        key: key,
                        type: type,
                        description: description,
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason,
                        block: blockAttribute,
                        variable: binding,
                        value: defaultValue) : nil
            }, diagnostic
        )
    }

    private static func extractEventModifier(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([EventMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var diagnostic: [Diagnostic] = []

        guard let blockAttribute = SyntaxUtils.extractAttribute(for: NativeModifierEventType, from: attributes) else {
            return nil
        }

        if attributes.count > 1 {
            diagnostic.append(Diagnostic(node: blockAttribute, message: DiagnosticType.multiAttributes))
        }

        let description = SyntaxUtils.extractDescription(from: blockAttribute) ?? ""
        let dataBindings = SyntaxUtils.extractDataBindings(from: blockAttribute) ?? []
        let deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute) ?? false
        let deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute) ?? ""

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let event = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""
                let function = binding.typeAnnotation?.type.as(FunctionTypeSyntax.self)
                    ?? binding.typeAnnotation?.type.as(OptionalTypeSyntax.self)?.wrappedType.as(TupleTypeSyntax.self)?
                        .elements.first?.type.as(FunctionTypeSyntax.self)
                let isOptionalFunction = binding.typeAnnotation?.type.as(OptionalTypeSyntax.self) != nil
                let parameters = function?.parameters ?? []

                if function == nil {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.functionTypeError))
                }

                if parameters.count != dataBindings.count {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.eventTypeMisMachParamCount))
                }

                return !event.isEmpty && function != nil
                    ? EventMeta(
                        kind: .action,
                        position: position,
                        event: event,
                        scope: nil,
                        description: description,
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason,
                        dataBindings: dataBindings,
                        isOptionalFunction: isOptionalFunction,
                        block: blockAttribute,
                        variable: binding
                    ) : nil
            }, diagnostic
        )
    }

    private static func extractExtraParam(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([ExtraParamMeta], [Diagnostic])? {
        var position = startPosition
        let diagnostic: [Diagnostic] = []

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let key = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""
                var type = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(IdentifierTypeSyntax.self)?.name.text ?? ""
                if type.isEmpty {
                    type = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(OptionalTypeSyntax.self)?
                        .wrappedType.as(IdentifierTypeSyntax.self)?.name.text ?? ""
                }
                return !key.isEmpty && !type.isEmpty
                    ? ExtraParamMeta(position: position, key: key, type: type, variable: binding) : nil
            }, diagnostic
        )
    }
}

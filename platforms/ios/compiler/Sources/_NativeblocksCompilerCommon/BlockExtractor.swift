import SwiftDiagnostics
import SwiftSyntax
import SwiftSyntaxBuilder

public enum BlockExtractor {
    static let NativeBlockDataType = "NativeBlockData"
    static let NativeBlockPropType = "NativeBlockProp"
    static let NativeBlockEventType = "NativeBlockEvent"
    static let NativeBlockSlotType = "NativeBlockSlot"

    public static func extractVariable(from structDecl: StructDeclSyntax) -> ([NativeMeta], [Diagnostic]) {
        var meta: [NativeMeta] = []
        var errors: [Diagnostic] = []
        var position = 0
        for member in structDecl.memberBlock.members {
            guard let varDecl = member.decl.as(VariableDeclSyntax.self) else { continue }
            position += 1

            let (type, _) = SyntaxUtils.getType(from: varDecl) ?? (nil, nil)

            switch type {
            case NativeBlockDataType:
                do {
                    guard let (block, blockErrors) = extractDataBlock(from: varDecl, startPosition: position) else {
                        continue
                    }
                    position = block.last?.position ?? position
                    errors.append(contentsOf: blockErrors)
                    meta.append(contentsOf: block)
                }
            case NativeBlockPropType:
                do {
                    guard let (block, blockErrors) = extractPropBlock(from: varDecl, startPosition: position) else {
                        continue
                    }
                    position = block.last?.position ?? position
                    errors.append(contentsOf: blockErrors)
                    meta.append(contentsOf: block)
                }
            case NativeBlockEventType:
                do {
                    guard let (block, blockErrors) = extractEventBlock(from: varDecl, startPosition: position) else {
                        continue
                    }
                    position = block.last?.position ?? position
                    errors.append(contentsOf: blockErrors)
                    meta.append(contentsOf: block)
                }
            case NativeBlockSlotType:
                do {
                    guard let (block, blockErrors) = extractSlotBlock(from: varDecl, startPosition: position) else {
                        continue
                    }
                    position = block.last?.position ?? position
                    errors.append(contentsOf: blockErrors)
                    meta.append(contentsOf: block)
                }
            default:
                guard let (block, blockErrors) = extractExtraParam(from: varDecl, startPosition: position) else {
                    continue
                }
                for param in block {
                    if param.key == "blockProps" && param.type == "BlockProps" {
                        meta.append(param)
                    }
                }
                position = block.last?.position ?? position
                errors.append(contentsOf: blockErrors)
            }
        }

        let dataBlocks = meta.compactMap { $0 as? DataMeta }
        let eventBlocks = meta.compactMap { $0 as? EventMeta }

        for event in eventBlocks {
            for binding in event.dataBinding {
                if dataBlocks.first(where: { data in data.key == binding }) == nil {
                    errors.append(Diagnostic(node: event.variable!, message: DiagnosticType.eventDataMissing))
                }
            }
        }
        return (meta, errors)
    }

    private static func extractDataBlock(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([DataMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var description = nil as String?
        var blockAttribute: AttributeSyntax?
        var diagnostic: [Diagnostic] = []
        var deprecated = false
        var deprecatedReason = nil as String?
        var defaultValue = ""

        blockAttribute = SyntaxUtils.extractAttribute(for: NativeBlockDataType, from: attributes)

        guard blockAttribute != nil else { return nil }

        if attributes.count > 1 {
            diagnostic.append(
                Diagnostic(
                    node: blockAttribute!,
                    message: DiagnosticType.multiAttributes))
        }

        description = SyntaxUtils.extractDescription(from: blockAttribute!) ?? ""
        deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute!) ?? false
        deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute!) ?? ""
        defaultValue = SyntaxUtils.extractDefaultValue(from: blockAttribute!) ?? ""

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let key = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""
                let type = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(IdentifierTypeSyntax.self)?.name.text ?? ""

                if !SyntaxUtils.isPrimitiveTypeSupported(type) {
                    diagnostic.append(Diagnostic(node: blockAttribute!, message: DiagnosticType.primitiveTypeSupported))
                }

                return !key.isEmpty && !type.isEmpty
                    ? DataMeta(
                        position: position,
                        key: key,
                        type: type,
                        description: description ?? "",
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason ?? "",
                        block: blockAttribute,
                        variable: binding,
                        value: defaultValue) : nil
            }, diagnostic
        )
    }

    private static func extractPropBlock(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([PropertyMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var description = ""
        var blockAttribute: AttributeSyntax?
        var diagnostic: [Diagnostic] = []
        var valuePicker = ""
        var valuePickerGroup = ""
        var valuePickerOptions: [ValuePickerOption] = []
        var deprecated = false
        var deprecatedReason = nil as String?
        var defaultValue = ""

        blockAttribute = SyntaxUtils.extractAttribute(for: NativeBlockPropType, from: attributes)

        guard blockAttribute != nil else { return nil }

        if attributes.count > 1 {
            diagnostic.append(
                Diagnostic(
                    node: blockAttribute!,
                    message: DiagnosticType.multiAttributes))
        }

        description = SyntaxUtils.extractDescription(from: blockAttribute!) ?? ""
        valuePicker = SyntaxUtils.extractValuePicker(from: blockAttribute!) ?? "TEXT_INPUT"
        valuePickerGroup = SyntaxUtils.extractValuePickerGroup(from: blockAttribute!) ?? "General"
        deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute!) ?? false
        deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute!) ?? ""
        defaultValue = SyntaxUtils.extractDefaultValue(from: blockAttribute!) ?? ""

        valuePickerOptions = SyntaxUtils.extractvaluePickerOptions(from: blockAttribute!) ?? []

        if varDecl.bindings.count > 1 {
            diagnostic.append(Diagnostic(node: blockAttribute!, message: DiagnosticType.singleVariableLimit))
        }

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let key = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""
                let type = SyntaxUtils.getType(typeAnnotation: binding.typeAnnotation)

                return !key.isEmpty && !type.isEmpty
                    ? PropertyMeta(
                        position: position,
                        key: key,
                        value: defaultValue,
                        type: type,
                        description: description,
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason ?? "",
                        valuePicker: valuePicker,
                        valuePickerOptions: valuePickerOptions,
                        valuePickerGroup: valuePickerGroup,
                        block: blockAttribute,
                        variable: binding) : nil
            }, diagnostic
        )
    }

    private static func extractEventBlock(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([EventMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var description = ""
        var dataBinding: [String] = []
        var isOptionalFunction = false
        var blockAttribute: AttributeSyntax?
        var diagnostic: [Diagnostic] = []
        var deprecated = false
        var deprecatedReason = nil as String?

        blockAttribute = SyntaxUtils.extractAttribute(for: NativeBlockEventType, from: attributes)

        guard blockAttribute != nil else { return nil }

        if attributes.count > 1 {
            diagnostic.append(
                Diagnostic(
                    node: blockAttribute!,
                    message: DiagnosticType.multiAttributes))
        }

        description = SyntaxUtils.extractDescription(from: blockAttribute!) ?? ""
        dataBinding = SyntaxUtils.extractDataBinding(from: blockAttribute!) ?? []
        deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute!) ?? false
        deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute!) ?? ""

        if varDecl.bindings.count > 1 {
            diagnostic.append(Diagnostic(node: blockAttribute!, message: DiagnosticType.singleVariableLimit))
        }

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let event = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""

                var function = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(
                    FunctionTypeSyntax.self)

                if function == nil {
                    function = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(
                        OptionalTypeSyntax.self)?.wrappedType.as(
                            TupleTypeSyntax.self)?.elements.as(TupleTypeElementListSyntax.self)?.first?.type.as(
                            FunctionTypeSyntax.self)
                    isOptionalFunction = function != nil
                }

                let parameters = function?.parameters ?? []

                if function == nil {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.functionTypeError))
                }

                if parameters.count != dataBinding.count {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.eventTypeMisMachParamCount))
                }

                return !event.isEmpty && function != nil
                    ? EventMeta(
                        kind: .block,
                        position: position,
                        event: event,
                        description: description,
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason ?? "",
                        dataBinding: dataBinding,
                        isOptionalFunction: isOptionalFunction,
                        block: blockAttribute,
                        variable: binding) : nil
            }, diagnostic
        )
    }

    private static func extractSlotBlock(from varDecl: VariableDeclSyntax, startPosition: Int) -> ([SlotMeta], [Diagnostic])? {
        var position = startPosition
        let attributes = varDecl.attributes
        var description = ""
        var blockAttribute: AttributeSyntax?
        var diagnostic: [Diagnostic] = []
        var deprecated = false
        var deprecatedReason = nil as String?

        blockAttribute = SyntaxUtils.extractAttribute(for: NativeBlockSlotType, from: attributes)

        guard blockAttribute != nil else { return nil }

        if attributes.count > 1 {
            diagnostic.append(
                Diagnostic(
                    node: blockAttribute!,
                    message: DiagnosticType.multiAttributes))
        }

        description = SyntaxUtils.extractDescription(from: blockAttribute!) ?? ""
        deprecated = SyntaxUtils.extractDeprecated(from: blockAttribute!) ?? false
        deprecatedReason = SyntaxUtils.extractDeprecatedReason(from: blockAttribute!) ?? ""

        if varDecl.bindings.count > 1 {
            diagnostic.append(Diagnostic(node: blockAttribute!, message: DiagnosticType.singleVariableLimit))
        }

        return (
            varDecl.bindings.compactMap { binding in
                position += 1
                let slot = binding.pattern.as(IdentifierPatternSyntax.self)?.identifier.text ?? ""

                var isOptionalFunction = false
                var function = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(
                    FunctionTypeSyntax.self)

                if function == nil {
                    function = binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(
                        OptionalTypeSyntax.self)?.wrappedType.as(
                            TupleTypeSyntax.self)?.elements.as(TupleTypeElementListSyntax.self)?.first?.type.as(
                            FunctionTypeSyntax.self)
                    isOptionalFunction = function != nil
                }
                let parameters = function?.parameters ?? []

                if function == nil {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.functionTypeError))
                }

                var hasBlockIndex = false
                var hasBlockScope = false

                if parameters.count > 2 {
                    diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.blockSlotParamLimit))
                } else if !parameters.isEmpty {
                    for (index, parameter) in parameters.enumerated() {

                        if let type = parameter.as(TupleTypeElementSyntax.self)?.type.as(
                            IdentifierTypeSyntax.self)?.name.text
                        {
                            if type == "BlockIndex" && index == 0 {
                                hasBlockIndex = true
                            } else if type == "Any" {
                                hasBlockScope = true
                            } else {
                                diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.blockSlotParamLimit))
                            }
                        } else {
                            diagnostic.append(Diagnostic(node: binding, message: DiagnosticType.blockSlotParamLimit))
                        }
                    }
                }

                return !slot.isEmpty
                    ? SlotMeta(
                        position: position,
                        slot: slot,
                        description: description,
                        deprecated: deprecated,
                        deprecatedReason: deprecatedReason ?? "",
                        hasBlockIndex: hasBlockIndex,
                        hasBlockScope: hasBlockScope,
                        isOptionalFunction: isOptionalFunction,
                        block: blockAttribute,
                        variable: binding) : nil
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
                    type =
                        binding.typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(OptionalTypeSyntax.self)?.wrappedType.as(
                            IdentifierTypeSyntax.self)?.name.text ?? ""
                }

                return !key.isEmpty && !type.isEmpty
                    ? ExtraParamMeta(
                        position: position,
                        key: key,
                        type: type,
                        variable: binding) : nil
            }, diagnostic
        )
    }
}

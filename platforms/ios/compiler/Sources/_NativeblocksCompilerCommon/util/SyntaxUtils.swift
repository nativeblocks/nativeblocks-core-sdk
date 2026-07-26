import SwiftSyntax

public enum SyntaxUtils {
    static func extractAttribute(for type: String, from attributes: AttributeListSyntax)
        -> AttributeSyntax?
    {
        for attribute in attributes {
            if let attr = attribute.as(AttributeSyntax.self),
                attr.attributeName.as(IdentifierTypeSyntax.self)?.name.text == type
            {
                return attr
            }
        }
        return nil
    }

    static func extractAttributes(from attributes: AttributeListSyntax)
        -> [String]
    {
        var attrs: [String] = []
        for attribute in attributes {
            if let attr = attribute.as(AttributeSyntax.self),
                let name = attr.attributeName.as(IdentifierTypeSyntax.self)?.name.text
            {
                attrs.append(name)
            }
        }
        return attrs
    }

    static func extractDescription(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "description" {
            if let segments = argument.expression.as(StringLiteralExprSyntax.self)?.segments.as(
                StringLiteralSegmentListSyntax.self)
            {
                return segments.first?.as(StringSegmentSyntax.self)?.content.text
            }
        }
        return nil
    }

    static func extractDeprecated(from attribute: AttributeSyntax) -> Bool? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "deprecated" {
            if let segments = argument.expression.as(BooleanLiteralExprSyntax.self) {
                return segments.literal.text == "true"
            }
        }
        return nil
    }

    static func extractDeprecatedReason(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "deprecatedReason" {
            if let segments = argument.expression.as(StringLiteralExprSyntax.self)?.segments.as(
                StringLiteralSegmentListSyntax.self)
            {
                return segments.first?.as(StringSegmentSyntax.self)?.content.text
            }
        }
        return nil
    }

    static func extractValuePicker(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "valuePicker" {
            return argument.expression.as(MemberAccessExprSyntax.self)?.declName.as(
                DeclReferenceExprSyntax.self)?.baseName.text
        }
        return nil
    }

    static func extractValuePickerGroup(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "valuePickerGroup" {
            return argument.expression.as(FunctionCallExprSyntax.self)?.arguments.first?.as(
                LabeledExprSyntax.self)?.expression.as(StringLiteralExprSyntax.self)?.segments.first?.as(
                    StringSegmentSyntax.self)?.content.text
        }
        return nil
    }

    static func extractvaluePickerOptions(from attribute: AttributeSyntax) -> [ValuePickerOption]? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        var options: [ValuePickerOption] = []
        for argument in arguments where argument.label?.text == "valuePickerOptions" {
            argument.expression.as(ArrayExprSyntax.self)?.elements.as(ArrayElementListSyntax.self)?
                .forEach { element in
                    let key = element.expression.as(FunctionCallExprSyntax.self)?.arguments.first?.as(
                        LabeledExprSyntax.self)?.expression.as(StringLiteralExprSyntax.self)?.segments.first?
                        .as(StringSegmentSyntax.self)?.content.text
                    let value = element.expression.as(FunctionCallExprSyntax.self)?.arguments.last?.as(
                        LabeledExprSyntax.self)?.expression.as(StringLiteralExprSyntax.self)?.segments.first?
                        .as(StringSegmentSyntax.self)?.content.text

                    if key != nil, value != nil {
                        options.append(ValuePickerOption(id: key!, text: value!))
                    }
                }
        }
        return options
    }

    static func extractDataBinding(from attribute: AttributeSyntax) -> [String]? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "dataBinding" {
            if let arrayElements = argument.expression.as(ArrayExprSyntax.self)?.elements {
                return arrayElements.compactMap { element in
                    element.expression.as(StringLiteralExprSyntax.self)?.segments.compactMap { segment in
                        segment.as(StringSegmentSyntax.self)?.content.text
                    }.joined()
                }
            }
        }
        return nil
    }

    static func extractThen(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else {
            return nil
        }

        for argument in arguments where argument.label?.text == "then" {
            if let declName = argument.expression.as(MemberAccessExprSyntax.self)?.declName.as(
                DeclReferenceExprSyntax.self)
            {
                return declName.baseName.text
            }
        }
        return nil
    }

    static func extractDefaultValue(from initializer: InitializerClauseSyntax?) -> String {
        guard let initializer = initializer?.value else { return "" }
        if let stringLiteral = initializer.as(StringLiteralExprSyntax.self)?.segments.first?.as(
            StringSegmentSyntax.self)?.content.text
        {
            return "\(stringLiteral)"
        } else if let intLiteral = initializer.as(IntegerLiteralExprSyntax.self)?.literal.text {
            return intLiteral
        } else if let floatLiteral = initializer.as(FloatLiteralExprSyntax.self)?.literal.text {
            return floatLiteral
        } else if let booleanLiteral = initializer.as(BooleanLiteralExprSyntax.self)?.description {
            return booleanLiteral
        } else if initializer.as(NilLiteralExprSyntax.self) != nil {
            return "nil"
        }
        return ""
    }

    static func extractDefaultValue(from attribute: AttributeSyntax) -> String? {
        guard let arguments = attribute.arguments?.as(LabeledExprListSyntax.self) else { return nil }
        for argument in arguments where argument.label?.text == "defaultValue" {
            if let segments = argument.expression.as(StringLiteralExprSyntax.self)?.segments.as(
                StringLiteralSegmentListSyntax.self)
            {
                return segments.first?.as(StringSegmentSyntax.self)?.content.text
            }
        }
        return nil
    }

    static func isPrimitiveTypeSupported(_ type: String) -> Bool {
        let supportedTypes: Set<String> = [
            "STRING", "BOOL", "INT", "INT64", "INT32", "INT16", "INT8", "UINT", "UINT64",
            "UINT32", "UINT16", "UINT8", "FLOAT", "FLOAT80", "FLOAT64", "FLOAT32",
            "FLOAT16", "CGFLOAT", "DOUBLE",
        ]
        return supportedTypes.contains(type.uppercased())
    }

    static func getType(from varDecl: VariableDeclSyntax, blockTypes: [String]) -> (
        String, AttributeSyntax
    )? {
        for type in blockTypes {
            if let attribute = extractAttribute(for: type, from: varDecl.attributes) {
                return (type, attribute)
            }
        }
        return nil
    }

    static func getType(from varDecl: VariableDeclSyntax) -> (String, AttributeSyntax)? {
        let attributes = varDecl.attributes
        for attribute in attributes {
            if let attr = attribute.as(AttributeSyntax.self) {
                let name = attr.attributeName.as(IdentifierTypeSyntax.self)?.name.text
                if name != nil {
                    return (name!, attr)
                }
            }
        }
        return nil
    }

    static func validateEventParams(
        _ function: FunctionTypeSyntax?, expectedCount: Int, binding: VariableDeclSyntax
    ) -> Bool {
        let parameters = function?.parameters ?? []
        return parameters.count == expectedCount
    }

    static func extractFunctionFromType(_ type: TypeSyntax?) -> (FunctionTypeSyntax?, Bool) {
        if let function = type?.as(FunctionTypeSyntax.self) {
            return (function, false)
        } else if let optionalFunction = type?.as(OptionalTypeSyntax.self)?.wrappedType.as(
            FunctionTypeSyntax.self)
        {
            return (optionalFunction, true)
        }
        return (nil, false)
    }

    static func getType(typeAnnotation: TypeAnnotationSyntax?) -> String {
        if let identifierType = typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(IdentifierTypeSyntax.self) {
            return identifierType.name.text
        } else if let memberType = typeAnnotation?.as(TypeAnnotationSyntax.self)?.type.as(MemberTypeSyntax.self) {
            return getType(memberType: memberType)
        } else {
            return ""
        }
    }

    private static func getType(memberType: MemberTypeSyntax?) -> String {
        guard let memberType = memberType else {
            return ""
        }

        if let nestedMemberType = memberType.baseType.as(MemberTypeSyntax.self) {
            let baseType = getType(memberType: nestedMemberType)
            let type = memberType.name.text
            return "\(baseType).\(type)"
        } else if let identifierType = memberType.baseType.as(IdentifierTypeSyntax.self) {
            let baseType = identifierType.name.text
            let type = memberType.name.text
            return "\(baseType).\(type)"
        } else {
            return memberType.name.text
        }
    }
}

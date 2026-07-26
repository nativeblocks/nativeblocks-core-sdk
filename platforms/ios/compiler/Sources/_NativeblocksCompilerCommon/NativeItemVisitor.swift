import Foundation
import SwiftParser
import SwiftSyntax

public class NativeItemVisitor: SyntaxVisitor {
    private var nativeItems: [Integration] = []

    public static func extractNatives(from sources: [String]) -> ([Integration], [Integration]) {
        let visitor = NativeItemVisitor(viewMode: SyntaxTreeViewMode.sourceAccurate)
        for source in sources {
            let sourceFile = Parser.parse(source: source)
            visitor.walk(sourceFile)
        }
        let natives = visitor.nativeItems
        let blocks = natives.filter { $0.kind == "BLOCK" && $0.syntaxStruct != nil }
        let actions = natives.filter { $0.kind == "ACTION" && $0.syntaxClass != nil }
        return (blocks, actions)
    }

    public override func visit(_ node: StructDeclSyntax) -> SyntaxVisitorContinueKind {
        let attributes = node.attributes
        if let attribute = findAttribute(name: "NativeBlock", from: attributes) {
            let structName = node.name.text
            let keyType = getStringValue(name: "keyType", from: attribute)
            let name = getStringValue(name: "name", from: attribute)
            let description = getStringValue(name: "description", from: attribute)
            let version = getIntValue(name: "version", from: attribute)
            let versionName = getStringValue(name: "versionName", from: attribute)
            let deprecated = getBoolValue(name: "deprecated", from: attribute)
            let deprecatedReason = getStringValue(name: "deprecatedReason", from: attribute)
            nativeItems.append(
                Integration(
                    declName: structName,
                    name: name!,
                    keyType: keyType!,
                    description: description!,
                    version: version ?? 1,
                    versionName: versionName ?? "",
                    deprecated: deprecated,
                    deprecatedReason: deprecatedReason ?? "",
                    syntaxStruct: node,
                    meta: [],
                    kind: "BLOCK"
                )
            )
        }
        return .skipChildren
    }

    public override func visit(_ node: ClassDeclSyntax) -> SyntaxVisitorContinueKind {
        let attributes = node.attributes
        if let attribute = findAttribute(name: "NativeAction", from: attributes) {
            let structName = node.name.text
            let keyType = getStringValue(name: "keyType", from: attribute)
            let name = getStringValue(name: "name", from: attribute)
            let description = getStringValue(name: "description", from: attribute)
            let version = getIntValue(name: "version", from: attribute)
            let versionName = getStringValue(name: "versionName", from: attribute)
            let deprecated = getBoolValue(name: "deprecated", from: attribute)
            let deprecatedReason = getStringValue(name: "deprecatedReason", from: attribute)
            nativeItems.append(
                Integration(
                    declName: structName,
                    name: name!,
                    keyType: keyType!,
                    description: description!,
                    version: version ?? 1,
                    versionName: versionName ?? "",
                    deprecated: deprecated,
                    deprecatedReason: deprecatedReason ?? "",
                    syntaxClass: node,
                    meta: [],
                    kind: "ACTION"
                )
            )
        }
        return .skipChildren
    }

    private func findAttribute(name: String, from attributes: AttributeListSyntax) -> AttributeSyntax? {
        for attribute in attributes {
            if let attributeIdentifier = attribute.as(AttributeSyntax.self)?.attributeName.as(
                IdentifierTypeSyntax.self
            ), attributeIdentifier.name.text == name {
                return attribute.as(AttributeSyntax.self)
            }
        }
        return nil
    }

    private func getStringValue(name: String, from attribute: AttributeSyntax) -> String? {
        var value: String? = nil
        attribute.arguments?.as(LabeledExprListSyntax.self)?.forEach { arg in
            if arg.label?.text == name {
                value =
                    arg.expression.as(
                        StringLiteralExprSyntax.self
                    )?.segments.as(
                        StringLiteralSegmentListSyntax.self
                    )?.first?.as(
                        StringSegmentSyntax.self
                    )?.content.text
            }
        }
        return value
    }

    private func getIntValue(name: String, from attribute: AttributeSyntax) -> Int? {
        var value: String? = nil
        attribute.arguments?.as(LabeledExprListSyntax.self)?.forEach { arg in
            if arg.label?.text == name {
                value =
                    arg.expression.as(
                        IntegerLiteralExprSyntax.self
                    )?.literal.text
            }
        }
        return Int(value ?? "")
    }

    private func getBoolValue(name: String, from attribute: AttributeSyntax) -> Bool {
        var value: String? = nil
        attribute.arguments?.as(LabeledExprListSyntax.self)?.forEach { arg in
            if arg.label?.text == name {
                value =
                    arg.expression.as(
                        BooleanLiteralExprSyntax.self
                    )?.literal.text
            }
        }
        return value == "true"
    }
}

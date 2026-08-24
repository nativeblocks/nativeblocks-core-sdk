import SwiftCompilerPlugin
import SwiftDiagnostics
import SwiftSyntax
import SwiftSyntaxBuilder
import SwiftSyntaxMacros
import _NativeblocksCompilerCommon

public struct NativeModifierMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        guard let structDecl = declaration.as(StructDeclSyntax.self) else {
            let structError = Diagnostic(node: declaration, message: DiagnosticType.notAStruct)
            context.diagnose(structError)
            return []
        }

        let (variables, diagnostic) = ModifierExtractor.extractVariable(from: structDecl)

        for error in diagnostic {
            context.diagnose(error)
        }

        let metaData = variables.compactMap { $0 as? DataMeta }
        let metaEvent = variables.compactMap { $0 as? EventMeta }
        let metaExtraParams = variables.compactMap { $0 as? ExtraParamMeta }

        let newStructDecl = try ModifierCreator.create(
            structName: structDecl.name.text,
            metaData: metaData,
            metaEvent: metaEvent,
            metaExtraParams: metaExtraParams
        )
        return [DeclSyntax(newStructDecl)]
    }
}

public struct NativeModifierDataMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeModifierEventMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

import SwiftCompilerPlugin
import SwiftDiagnostics
import SwiftSyntax
import SwiftSyntaxBuilder
import SwiftSyntaxMacros
import _NativeblocksCompilerCommon

public struct NativeActionMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        guard let structDecl = declaration.as(ClassDeclSyntax.self) else {
            let structError = Diagnostic(node: declaration, message: DiagnosticType.notAClass)
            context.diagnose(structError)
            return []
        }

        let (variables, diagnostic) = ActionExtractor.extractVariable(from: structDecl)

        for error in diagnostic {
            context.diagnose(error)
        }

        let metaData = variables.compactMap { $0 as? DataMeta }
        let metaProp = variables.compactMap { $0 as? PropertyMeta }
        let metaEvent = variables.compactMap { $0 as? EventMeta }
        let actionInfo = variables.compactMap { $0 as? ActionMeta }.first
        let metaExtraParams = variables.compactMap { $0 as? ExtraParamMeta }

        let newStructDecl = try ActionCreator.create(
            structName: structDecl.name.text,
            actionInfo: actionInfo,
            metaData: metaData,
            metaProp: metaProp,
            metaEvent: metaEvent,
            metaExtraParams: metaExtraParams
        )
        return [DeclSyntax(newStructDecl)]
    }
}

public struct NativeActionFunctionMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeActionParameterMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeActionDataMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeActionPropMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeActionEventMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

import SwiftCompilerPlugin
import SwiftDiagnostics
import SwiftSyntax
import SwiftSyntaxBuilder
import SwiftSyntaxMacros
import _NativeblocksCompilerCommon

public struct NativeBlockMacro: PeerMacro {
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

        let (variables, diagnostic) = BlockExtractor.extractVariable(from: structDecl)

        for error in diagnostic {
            context.diagnose(error)
        }

        let metaData = variables.compactMap { $0 as? DataMeta }
        let metaProp = variables.compactMap { $0 as? PropertyMeta }
        let metaEvent = variables.compactMap { $0 as? EventMeta }
        let metaSlot = variables.compactMap { $0 as? SlotMeta }
        let metaExtraParams = variables.compactMap { $0 as? ExtraParamMeta }

        let newStructDecl = try BlockCreator.create(
            structName: structDecl.name.text,
            metaData: metaData,
            metaProp: metaProp,
            metaEvent: metaEvent,
            metaSlot: metaSlot,
            metaExtraParams: metaExtraParams
        )
        return [DeclSyntax(newStructDecl)]
    }
}

public struct NativeBlockDataMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeBlockPropMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeBlockEventMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

public struct NativeBlockSlotMacro: PeerMacro {
    public static func expansion(
        of node: SwiftSyntax.AttributeSyntax,
        providingPeersOf declaration: some SwiftSyntax.DeclSyntaxProtocol,
        in context: some SwiftSyntaxMacros.MacroExpansionContext
    ) throws -> [SwiftSyntax.DeclSyntax] {
        return []
    }
}

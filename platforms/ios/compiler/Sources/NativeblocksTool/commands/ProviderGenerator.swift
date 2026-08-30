import Foundation
import SwiftParser
import SwiftSyntax
import _NativeblocksCompilerCommon

public class ProviderGenerator {
    public var actionProviderCode: String?
    public var blockProviderCode: String?
    public var modifierProviderCode: String?

    private let prefix: String

    public init(prefix: String) {
        self.prefix = ProviderGenerator.generateName(prefix: prefix)
    }

    public func generate(from files: [String]) throws {
        print("Generate providers...")

        let (blocks, actions, modifiers) = NativeItemVisitor.extractNatives(from: files)

        if blocks.isEmpty && actions.isEmpty && modifiers.isEmpty {
            print("There is no actions, blocks or modifiers to generate providers")
            return
        }

        if !actions.isEmpty {
            actionProviderCode = try createActionProvider(prefix: prefix, actions: actions).formatted().description
        }

        if !blocks.isEmpty {
            blockProviderCode = try createBlockProvider(prefix: prefix, blocks: blocks).formatted().description
        }

        if !modifiers.isEmpty {
            modifierProviderCode = try createModifierProvider(prefix: prefix, modifiers: modifiers).formatted().description
        }
    }

    public func save(to directory: String) throws {
        print("Save providers...")

        if actionProviderCode == nil && blockProviderCode == nil && modifierProviderCode == nil {
            print("There is no actions, blocks or modifiers to save providers")
            return
        }

        let actionFilePath = directory + "/\(prefix)ActionProvider.swift"
        let blockFilePath = directory + "/\(prefix)BlockProvider.swift"
        let modifierFilePath = directory + "/\(prefix)ModifierProvider.swift"

        try actionProviderCode?.write(toFile: actionFilePath, atomically: true, encoding: .utf8)
        try blockProviderCode?.write(toFile: blockFilePath, atomically: true, encoding: .utf8)
        try modifierProviderCode?.write(toFile: modifierFilePath, atomically: true, encoding: .utf8)

        if actionProviderCode != nil {
            print("exported File: \(actionFilePath) =>")
            print(String(actionProviderCode!))
        }

        if blockProviderCode != nil {
            print("exported File: \(blockFilePath) =>")
            print(String(blockProviderCode!))
        }

        if modifierProviderCode != nil {
            print("exported File: \(modifierFilePath) =>")
            print(String(modifierProviderCode!))
        }
    }

    static func generateName(prefix: String?) -> String {
        guard ((prefix?.isEmpty) != nil) == true else { return "Default" }
        let firstCharacter = prefix!.prefix(1).uppercased()
        let remainingCharacters = prefix!.dropFirst()
        return firstCharacter + remainingCharacters
    }

    func createBlockProvider(prefix: String, blocks: [Integration]) throws -> SourceFileSyntax {
        return try SourceFileSyntax {
            """
            import NativeblocksRuntime
            """
            try ClassDeclSyntax("public class \(raw: prefix)BlockProvider") {
                try FunctionDeclSyntax("public static func provideBlocks(name: String = \"default\")") {
                    for block in blocks {
                        if block.describing {
                            """
                            NativeblocksManager.getInstance(name: name).provideBlock(
                                blockType: "\(raw: block.keyType)",
                                block: .describing { blockContext, describeScope in
                                    \(raw: block.declName)Block(blockContext: blockContext, describeScope: describeScope)
                                }
                            )
                            """
                        } else {
                            """
                            NativeblocksManager.getInstance(name: name).provideBlock(
                                blockType: "\(raw: block.keyType)",
                                block: .rendering { blockContext in
                                    \(raw: block.declName)Block(blockContext: blockContext)
                                }
                            )
                            """
                        }
                    }
                }
            }
        }
    }

    func createModifierProvider(prefix: String, modifiers: [Integration]) throws -> SourceFileSyntax {
        return try SourceFileSyntax {
            """
            import NativeblocksRuntime
            import SwiftUI
            """
            try ClassDeclSyntax("public class \(raw: prefix)ModifierProvider") {
                try FunctionDeclSyntax("public static func provideModifiers(name: String = \"default\")") {
                    for modifier in modifiers {
                        """
                        NativeblocksManager.getInstance(name: name).provideModifier(modifierType: "\(raw: modifier.keyType)") { content, modifierContext in
                            AnyView(content.modifier(\(raw: modifier.declName)Modifier(modifierContext: modifierContext)))
                        }
                        """
                    }
                }
            }
        }
    }

    func createActionProvider(prefix: String, actions: [Integration]) throws -> SourceFileSyntax {
        return try SourceFileSyntax {
            """
            import NativeblocksRuntime
            """
            try ClassDeclSyntax("public class \(raw: prefix)ActionProvider") {
                let arguments = actions.map { action in
                    """
                    \(refineActionArgumentName(name: action.declName)) : \(action.declName)
                    """
                }.joined(separator: " ,")

                try FunctionDeclSyntax("public static func provideActions(name: String = \"default\" ,\(raw: arguments))") {
                    for action in actions {
                        """
                        NativeblocksManager.getInstance(name: name).provideAction(actionType: "\(raw: action.keyType)", action: \(raw: action.declName)Action(action: \(raw: refineActionArgumentName(name: action.declName))))
                        """
                    }
                }
            }
        }
    }

    func refineActionArgumentName(name: String) -> String {
        let firstCharacter = name.prefix(1).lowercased()
        let remainingCharacters = name.dropFirst()
        return firstCharacter + remainingCharacters
    }
}

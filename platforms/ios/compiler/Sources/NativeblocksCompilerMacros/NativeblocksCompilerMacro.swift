import SwiftCompilerPlugin
import SwiftSyntaxMacros

@main
struct NativeBlocksPlugin: CompilerPlugin {
    let providingMacros: [Macro.Type] = [
        NativeBlockMacro.self,
        NativeBlockDataMacro.self,
        NativeBlockPropMacro.self,
        NativeBlockEventMacro.self,
        NativeBlockSlotMacro.self,
        NativeActionMacro.self,
        NativeActionParameterMacro.self,
        NativeActionFunctionMacro.self,
        NativeActionDataMacro.self,
        NativeActionPropMacro.self,
        NativeActionEventMacro.self,
    ]
}

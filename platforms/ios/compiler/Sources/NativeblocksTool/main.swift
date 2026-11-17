import Foundation

print("arguments: \(CommandLine.arguments)")
do {
    let tool = try NativeblocksToolExecutor(CommandLine.arguments)
    try tool.execute()
} catch {
    print("NativeblocksTool Error: \(error)")
    exit(1)
}

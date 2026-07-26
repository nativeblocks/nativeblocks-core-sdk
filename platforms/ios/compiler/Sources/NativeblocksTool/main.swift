import Foundation

print("arguments: \(CommandLine.arguments)")
do {
    let tool = try NativeblocksToolExecutor(CommandLine.arguments)
    try tool.execute()
} catch {
    guard let errorModel = error as? ErrorModel else {
        print("NativeblocksTool Error: \(error)")
        exit(1)
    }

    print("NativeblocksTool Error:\(errorModel.message ?? "")")
    exit(1)
}

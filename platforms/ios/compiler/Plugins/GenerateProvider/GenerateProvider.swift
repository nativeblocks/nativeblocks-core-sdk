import Foundation
import PackagePlugin

@main
struct GenerateProvider: CommandPlugin {
    func performCommand(
        context: PluginContext,
        arguments: [String]
    ) throws {
        let tool = try context.tool(named: "NativeblocksTool")
        let toolURL = URL(fileURLWithPath: tool.path.string)
        let command = "generate-provider"
        var processArguments = arguments

        if processArguments.filter({ arg in
            arg == "--directory"
        }).count == 0 {
            processArguments.append(contentsOf: ["--directory", context.package.directory.string])
        }
        try callNativeblocksTool(toolURL: toolURL, command: command, arguments: processArguments)
    }
}

#if canImport(XcodeProjectPlugin)
    import XcodeProjectPlugin

    extension GenerateProvider: XcodeCommandPlugin {
        func performCommand(context: XcodePluginContext, arguments: [String]) throws {
            let tool = try context.tool(named: "NativeblocksTool")
            let toolURL = URL(fileURLWithPath: tool.path.string)
            var processArguments = arguments
            let command = "generate-provider"

            if processArguments.filter({ arg in
                arg == "--directory"
            }).count == 0 {
                processArguments.append(contentsOf: ["--directory", context.xcodeProject.directory.string])
            }

            try callNativeblocksTool(toolURL: toolURL, command: command, arguments: processArguments)
        }
    }

#endif

func callNativeblocksTool(toolURL: URL, command: String, arguments: [String]) throws {
    let process = Process()
    process.executableURL = toolURL
    process.arguments = [command] + arguments

    print("call NativeblocksTool:\(command) \(arguments)")

    try process.run()
    process.waitUntilExit()

    if process.terminationStatus == 0 {
        print("Successfully ran call NativeblocksTool with arguments.")
    } else {
        print("Failed to run call NativeblocksTool. Exit code: \(process.terminationStatus)")
    }
}

import Foundation
import PackagePlugin

@main
struct Sync: CommandPlugin {
    func performCommand(
        context: PluginContext,
        arguments: [String]
    ) throws {
        let tool = try context.tool(named: "NativeblocksTool")
        let toolURL = URL(fileURLWithPath: tool.path.string)
        let command = "sync"
        var processArguments = arguments

        guard let config = try readNativeblocksConfig(rootPath: context.package.directory.string) else {
            return
        }
        processArguments.append(contentsOf: ["--endpoint", config.endpoint])
        processArguments.append(contentsOf: ["--authToken", config.authToken])
        processArguments.append(contentsOf: ["--organizationId", config.organizationId])

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

    extension Sync: XcodeCommandPlugin {
        func performCommand(context: XcodePluginContext, arguments: [String]) throws {
            let tool = try context.tool(named: "NativeblocksTool")
            let toolURL = URL(fileURLWithPath: tool.path.string)
            var processArguments = arguments
            let command = "sync"

            guard let config = try readNativeblocksConfig(rootPath: context.xcodeProject.directory.string)
            else {
                return
            }
            processArguments.append(contentsOf: ["--endpoint", config.endpoint])
            processArguments.append(contentsOf: ["--authToken", config.authToken])
            processArguments.append(contentsOf: ["--organizationId", config.organizationId])

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
        Diagnostics.error(
            "Failed to run call NativeblocksTool. Exit code: \(process.terminationStatus)")
    }
}

func readNativeblocksConfig(rootPath: String) throws -> NativeblocksConfig? {
    let configFilePath = rootPath + "/nativeblocks.json"

    let fileManager = FileManager.default

    guard fileManager.fileExists(atPath: configFilePath) else {
        Diagnostics.error("nativeblocks.json file not found at the root of the project.")
        return nil
    }
    let fileURL = URL(fileURLWithPath: configFilePath)
    do {
        let data = try Data(contentsOf: fileURL)

        let decoder = JSONDecoder()
        return try decoder.decode(NativeblocksConfig.self, from: data)

    } catch {
        Diagnostics.error("Failed to read or decode nativeblocks.json: \(error.localizedDescription)")
    }
    return nil
}

struct NativeblocksConfig: Codable {
    var endpoint: String
    var authToken: String
    var organizationId: String
}

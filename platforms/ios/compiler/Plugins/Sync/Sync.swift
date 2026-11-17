import Foundation
import PackagePlugin

@main
struct Sync: CommandPlugin {
    func performCommand(
        context: PluginContext,
        arguments: [String]
    ) throws {
        let tool = try context.tool(named: "NativeblocksTool")
        try sync(
            toolURL: URL(fileURLWithPath: tool.path.string),
            rootPath: context.package.directory.string,
            workPath: context.pluginWorkDirectory.string,
            arguments: arguments)
    }
}

#if canImport(XcodeProjectPlugin)
    import XcodeProjectPlugin

    extension Sync: XcodeCommandPlugin {
        func performCommand(context: XcodePluginContext, arguments: [String]) throws {
            let tool = try context.tool(named: "NativeblocksTool")
            try sync(
                toolURL: URL(fileURLWithPath: tool.path.string),
                rootPath: context.xcodeProject.directory.string,
                workPath: context.pluginWorkDirectory.string,
                arguments: arguments)
        }
    }

#endif

private func sync(toolURL: URL, rootPath: String, workPath: String, arguments: [String]) throws {
    if FileManager.default.fileExists(atPath: rootPath + "/nativeblocks.json") {
        Diagnostics.warning(
            "nativeblocks.json is not used anymore, sync uses your Nativeblocks CLI login. Delete it, it contains an auth token."
        )
    }

    guard let cli = findCli() else {
        Diagnostics.error(
            """
            Nativeblocks CLI not found. Sync needs it to upload your integrations.
            Install it: curl -fsSL https://nativeblocks.io/download/cli/installer.sh | bash
            Then run `nativeblocks auth login` and `nativeblocks organization set`.
            """
        )
        return
    }

    let integrationsPath = workPath + "/integrations"
    var toolArguments = arguments
    if !toolArguments.contains("--directory") {
        toolArguments.append(contentsOf: ["--directory", rootPath])
    }
    toolArguments.append(contentsOf: ["--output", integrationsPath])
    try? FileManager.default.removeItem(atPath: integrationsPath)
    let schema = try run(toolURL, ["prepare-schema"] + toolArguments, in: rootPath)
    print(schema.output)
    guard schema.status == 0 else {
        Diagnostics.error("Failed to run NativeblocksTool. Exit code: \(schema.status)")
        return
    }

    let integrations = findIntegrations(in: integrationsPath)
    guard !integrations.isEmpty else {
        Diagnostics.error("There is no integration in \(rootPath), please make sure the implementation is correct")
        return
    }

    for integration in integrations {
        let name = String(integration.dropFirst(integrationsPath.count + 1))
        let result = try run(cli, ["integration", "sync", "--directory", integration], in: rootPath)
        guard result.status == 0 else {
            Diagnostics.error("Failed to upload \(name): \(cliError(result.output))")
            return
        }
        print("Integration \(name) has been uploaded successfully")
    }
}

private func findCli() -> URL? {
    let home = FileManager.default.homeDirectoryForCurrentUser.path
    let pathDirs = (ProcessInfo.processInfo.environment["PATH"] ?? "")
        .split(separator: ":")
        .map(String.init)
    let dirs = pathDirs + [home + "/.nativeblocks/bin", "/opt/homebrew/bin", "/usr/local/bin"]
    return dirs
        .map { URL(fileURLWithPath: $0).appendingPathComponent("nativeblocks") }
        .first { FileManager.default.isExecutableFile(atPath: $0.path) }
}

private func findIntegrations(in path: String) -> [String] {
    guard let files = FileManager.default.enumerator(atPath: path) else {
        return []
    }
    return files
        .compactMap { $0 as? String }
        .filter { ($0 as NSString).lastPathComponent == "integration.json" }
        .map { path + "/" + ($0 as NSString).deletingLastPathComponent }
        .sorted()
}

private func cliError(_ output: String) -> String {
    let error = output
        .split(separator: "\n")
        .first { $0.hasPrefix("Error:") }
        .map { $0.dropFirst("Error:".count).trimmingCharacters(in: .whitespaces) }
        ?? output.trimmingCharacters(in: .whitespacesAndNewlines)
    if error.contains("no token") {
        return error + ". Make sure the Nativeblocks CLI is 1.9.2 or newer with `nativeblocks update`."
    }
    return error
}

private func run(_ executable: URL, _ arguments: [String], in directory: String) throws -> (status: Int32, output: String) {
    let process = Process()
    let pipe = Pipe()
    process.executableURL = executable
    process.arguments = arguments
    process.currentDirectoryURL = URL(fileURLWithPath: directory)
    process.standardOutput = pipe
    process.standardError = pipe

    try process.run()
    let data = pipe.fileHandleForReading.readDataToEndOfFile()
    process.waitUntilExit()
    return (process.terminationStatus, String(decoding: data, as: UTF8.self))
}

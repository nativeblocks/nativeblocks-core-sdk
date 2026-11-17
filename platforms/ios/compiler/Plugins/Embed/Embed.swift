import Foundation
import PackagePlugin

@main
struct Embed: CommandPlugin {
    func performCommand(
        context: PluginContext,
        arguments: [String]
    ) throws {
        Diagnostics.error("Embed only works on an Xcode app project. Swift packages can't ship embedded content.")
    }
}

#if canImport(XcodeProjectPlugin)
    import XcodeProjectPlugin

    extension Embed: XcodeCommandPlugin {
        func performCommand(context: XcodePluginContext, arguments: [String]) throws {
            try embed(projectPath: context.xcodeProject.directory.string)
        }
    }

#endif

private func embed(projectPath: String) throws {
    guard let cli = findCli() else {
        Diagnostics.error(
            """
            Nativeblocks CLI not found. Embed needs it to download your frames.
            Install it: curl -fsSL https://nativeblocks.io/download/cli/installer.sh | bash
            Then run `nativeblocks auth login`, `nativeblocks organization set` and `nativeblocks project set`.
            """
        )
        return
    }

    let result = try run(cli, ["embed", "sync", "--platform", "IOS", "--target", projectPath], in: projectPath)
    guard result.status == 0 else {
        Diagnostics.error("Failed to embed: \(cliError(result.output))")
        return
    }
    print(result.output.trimmingCharacters(in: .whitespacesAndNewlines))
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
